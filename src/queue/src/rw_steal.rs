//! Work stealing queue implementation.

use std::cell::UnsafeCell;
use std::sync::{Arc, RwLock};

const QUEUE_CAPACITY: u16 = 256;
const MASK: u16 = QUEUE_CAPACITY - 1;

/// The inner queue that holds the current thread owns
struct Queue<T>(Arc<RwLock<Inner<T>>>);

impl<T> !Send for Queue<T> {}

struct Steal<T>(Arc<RwLock<Inner<T>>>);

unsafe impl<T> Send for Inner<T> {}
unsafe impl<T> Sync for Inner<T> {}

struct Inner<T> {
    head: u16,
    tail: u16,
    items: Box<[UnsafeCell<Option<T>>; QUEUE_CAPACITY as usize]>,
}

impl<T> Queue<T> {
    pub fn push(&self, item: T) -> Result<(), &'static str> {
        let mut inner = self.0.write().unwrap();

        if inner.is_full() {
            return Err("queue full");
        }

        // We need to hold the lock longer because it is possible when
        // perform the set without holding the lock, the take at the
        // current `inner.tail` might not have finished yet.

        // drop(inner);
        // let inner = self.0.read().unwrap();
        inner.set(inner.tail, item)?;

        inner.tail = inner.tail.wrapping_add(1);
        Ok(())
    }

    /// Pop an item from the queue, None if empty
    pub fn pop(&self) -> Option<T> {
        let mut inner = self.0.write().unwrap();
        if inner.is_empty() {
            return None;
        }

        let head = inner.head;
        inner.head = inner.head.wrapping_add(1);
        drop(inner);

        let inner = self.0.read().unwrap();
        Some(inner.take(head))
    }

    pub fn size(&self) -> u16 {
        self.0.read().unwrap().size()
    }
}

impl<T> Steal<T> {
    pub fn steal(&self) -> Option<T> {
        let mut inner = self.0.write().unwrap();
        if inner.is_empty() {
            return None;
        }

        let head = inner.head;
        inner.head = inner.head.wrapping_add(1);
        drop(inner);

        let inner = self.0.read().unwrap();
        Some(inner.take(head))
    }
}

impl<T> Inner<T> {
    fn new() -> Self {
        let mut buffer = Vec::with_capacity(QUEUE_CAPACITY as usize);

        for _ in 0..QUEUE_CAPACITY {
            buffer.push(UnsafeCell::new(None));
        }

        Inner {
            head: 0,
            tail: 0,
            items: make_fixed_size(buffer.into_boxed_slice()),
        }
    }

    fn is_full(&self) -> bool {
        self.size() == QUEUE_CAPACITY
    }

    fn is_empty(&self) -> bool {
        self.tail == self.head
    }

    fn size(&self) -> u16 {
        self.tail.wrapping_sub(self.head)
    }

    fn take(&self, i: u16) -> T {
        let i = (i & MASK) as usize;
        let cell = self.items.get(i).unwrap();
        unsafe {
            let r: &mut Option<T> = &mut *cell.get();
            r.take().unwrap()
        }
    }

    fn set(&self, i: u16, item: T) -> Result<(), &'static str> {
        let i = (i & MASK) as usize;
        let cell = self.items.get(i).unwrap();
        unsafe {
            let r: &mut Option<T> = &mut *cell.get();
            if r.is_some() {
                return Err("not consumed");
            }
            r.replace(item);
        }
        Ok(())
    }
}

fn make_fixed_size<T>(buffer: Box<[T]>) -> Box<[T; QUEUE_CAPACITY as usize]> {
    assert_eq!(buffer.len(), QUEUE_CAPACITY as usize);

    // safety: We check that the length is correct.
    unsafe { Box::from_raw(Box::into_raw(buffer).cast()) }
}

#[cfg(test)]
mod tests {
    use crate::rw_steal::{Inner, Queue, Steal, QUEUE_CAPACITY};
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex, RwLock};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use std::time::Instant;

    #[derive(Eq, PartialEq, Debug)]
    struct TestItem {}

    #[derive(Eq, PartialEq, Debug, Hash)]
    struct IncrItem {
        i: usize,
    }

    #[test]
    fn test_sub() {
        let a = 0u16.wrapping_sub(u16::MAX);
        let m = u16::MAX;
        println!("{a:?}, {m:?}");
    }

    #[test]
    fn test_queue() {
        let inner = Inner::new();
        let queue = Queue::<TestItem>(Arc::new(RwLock::new(inner)));

        queue.push(TestItem {}).unwrap();
        assert_eq!(queue.size(), 1);

        let i = queue.pop().unwrap();
        assert_eq!(i, TestItem {});
        assert!(queue.pop().is_none());
    }

    #[test]
    fn test_queue_full() {
        let inner = Inner::new();
        let queue = Queue::<TestItem>(Arc::new(RwLock::new(inner)));

        for _ in 0..QUEUE_CAPACITY {
            queue.push(TestItem {}).unwrap();
        }
        assert_eq!(queue.size(), QUEUE_CAPACITY);
        assert!(queue.push(TestItem {}).is_err());

        queue.pop().unwrap();

        queue.push(TestItem {}).unwrap();
        assert_eq!(queue.size(), QUEUE_CAPACITY);
    }

    #[test]
    fn loops() {
        let inner = Inner::new();
        let queue = Queue::<TestItem>(Arc::new(RwLock::new(inner)));

        queue.push(TestItem {}).unwrap();
        assert_eq!(queue.size(), 1);

        let i = queue.pop().unwrap();
        assert_eq!(i, TestItem {});
        assert!(queue.pop().is_none());

        let loops = 100;
        for _ in 0..loops {
            for _ in 0..QUEUE_CAPACITY {
                queue.push(TestItem {}).unwrap();
            }
            for _ in 0..QUEUE_CAPACITY {
                queue.pop();
            }
        }
    }

    #[test]
    fn with_steal() {
        let start_time = Instant::now();

        let inner = Arc::new(RwLock::new(Inner::new()));
        let queue = Queue::<IncrItem>(inner.clone());

        let finished_push = Arc::new(AtomicBool::new(false));

        let set = Arc::new(Mutex::new(HashSet::new()));

        let num_steals = 5;
        let mut handles = vec![];
        for _ in 0..num_steals {
            let steal = Steal::<IncrItem>(inner.clone());
            let s = set.clone();
            let finished_push_cloned = finished_push.clone();

            let h = thread::spawn(move || {
                let mut m = HashSet::new();

                loop {
                    if finished_push_cloned.load(Ordering::Acquire) {
                        loop {
                            let v = steal.steal();
                            if v.is_none() {
                                break;
                            }
                            m.insert(v.unwrap());
                        }
                        break;
                    }

                    for _ in 0..1000 {
                        let v = steal.steal();
                        if v.is_none() {
                            continue;
                        }
                        m.insert(v.unwrap());
                    }
                }


                let mut k = s.lock().unwrap();
                for num in m {
                    if k.contains(&num) {
                        println!("{k:?}-{num:?}");
                        panic!("duplicate");
                    }
                    k.insert(num);
                }
            });

            handles.push(h);
        }

        let mut i = 0;
        for _ in 0..500000 {
            match queue.push(IncrItem { i }) {
                Ok(_) => i += 1,
                Err(_) => {
                    continue
                },
            };
        }

        finished_push.store(true, Ordering::Release);

        for h in handles {
            h.join().unwrap();
        }

        let duration = start_time.elapsed().as_millis();
        println!("rw lock per loop: {:}ms", duration);

        let s = set.lock().unwrap();
        for n in 0..i {
            assert!(s.contains(&IncrItem{i: n}));
        }
    }
}
