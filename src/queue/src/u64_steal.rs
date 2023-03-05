//! Work stealing queue implementation.

use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicU16, AtomicU64, Ordering};
use std::sync::Arc;

const QUEUE_CAPACITY: u64 = 256;
const MASK: u64 = QUEUE_CAPACITY - 1;
const UNPACK_MASK: u32 = u16::MAX as u32;

/// The inner queue that holds the current thread owns
struct Queue<T>(Arc<Inner<T>>);

impl<T> !Send for Queue<T> {}

struct Steal<T>(Arc<Inner<T>>);

unsafe impl<T> Send for Inner<T> {}
unsafe impl<T> Sync for Inner<T> {}

struct Inner<T> {
    head: AtomicU64,
    tail: AtomicU64,
    items: Box<[UnsafeCell<Option<T>>; QUEUE_CAPACITY as usize]>,
}

impl<T> Queue<T> {
    pub fn push(&self, item: T) -> Result<(), &'static str> {
        self.0.push(item)
    }

    /// Pop an item from the queue, None if empty
    pub fn pop(&self) -> Option<T> {
        self.0.pop()
    }

    pub fn size(&self) -> u64 {
        let tail = self.0.tail.load(Ordering::Acquire);
        let head = self.0.head.load(Ordering::Acquire);
        tail.wrapping_sub(head)
    }
}

impl<T> Steal<T> {
    pub fn steal(&self) -> Option<T> {
        self.0.pop()
    }
}

impl<T> Inner<T> {
    fn new() -> Self {
        let mut buffer = Vec::with_capacity(QUEUE_CAPACITY as usize);

        for _ in 0..QUEUE_CAPACITY {
            buffer.push(UnsafeCell::new(None));
        }

        Inner {
            head: AtomicU64::new(0),
            tail: AtomicU64::new(0),
            items: make_fixed_size(buffer.into_boxed_slice()),
        }
    }

    fn push(&self, item: T) -> Result<(), &'static str> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);

        if head > tail {
            panic!("push: {head:?} and {tail:?}")
        }

        if is_full(head, tail) {
            return Err("queue full");
        }

        self.set(tail, item);
        self.tail.store(tail.wrapping_add(1), Ordering::Release);

        Ok(())
    }

    /// First version of implementation. If we run it, it will panic with "stale head read".
    /// Acquire-Release ordering does not guarantee the latest value will be read. When the
    /// `compare_exchange` fails, the latest value (in the eyes of the current thread) is
    /// returned. If we do a load with Acquire, it won't guarantee the updates by the other threads
    /// will be picked up, it is possible to return a stale value.
    ///
    ///   Thead 0 Push       Thread A Pop              Thread B Pop
    ///       Tail          Tail     Head               Tail     Head
    ///        | L=0          |        | L=0              |        | L=0
    ///        |              | L=0    |                  | L=0    |
    ///        | L=1          |        |                  |        | S=1 (OK)
    ///        |              |        | S=1(Err,RET=1)   |        |
    ///        |              |        | L=0/1            |        | L=1
    ///        | L=2          | L=0/1  |                  | L=0/1  |
    ///        |              |        |                  |        |
    fn pop1(&self) -> Option<T> {
        let mut act = u64::MAX;

        let i = loop {
            let tail = self.tail.load(Ordering::Acquire);
            let head = self.head.load(Ordering::Acquire);

            if act == head {
                panic!("stale head read");
            }

            if head > tail {
                panic!("{head:?} and {tail:?}")
            }

            if is_empty(head, tail) {
                return None;
            }

            match self.head.compare_exchange(
                head,
                head.checked_add(1).unwrap(),
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => break head,
                Err(actual) => {
                    act = actual;
                },
            }
        };

        Some(self.take(i))
    }

    ///   Thead 0 Push       Thread A Pop              Thread B Pop
    ///       Tail          Tail     Head               Tail     Head
    ///        | L=0          |        | L=0              |        | L=0
    ///        |              | L=0    |                  | L=0    |
    ///        | L=1          |        |                  |        |
    ///        |              RETURN NONE                 RETURN NONE
    ///        |              |        | L=0              |        | L=0
    ///        |              | L=1    |                  | L=1    |
    ///        |              |        |                  |        | S=1 (OK)
    ///        |              |        | S=1(RET=1)       |        |
    ///        |              |        | 1                |        | L=1
    ///        | L=2          |        |                  |        |
    ///        |              | L=1/2  |                  | L=1/2  |
    ///        |              |        |                  |        |
    fn pop(&self) -> Option<T> {
        let mut head = self.head.load(Ordering::Acquire);

        let i = loop {
            let tail = self.tail.load(Ordering::Acquire);

            if head > tail {
                panic!("{head:?} and {tail:?}")
            }

            if is_empty(head, tail) {
                return None;
            }

            match self.head.compare_exchange(
                head,
                head.checked_add(1).unwrap(),
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => break head,
                Err(actual) => head = actual,
            }
        };

        Some(self.take(i))
    }

    fn take(&self, i: u64) -> T {
        loop {
            let idx = (i & MASK) as usize;
            let ptr = self.items.get(idx).unwrap().get();
            unsafe {
                let p = &mut *ptr;
                // if p.is_none() { continue }
                return p.take().unwrap();
            }
        }
    }

    fn set(&self, i: u64, item: T) {
        loop {
            let idx = (i & MASK) as usize;
            let ptr = self.items.get(idx).unwrap().get();
            unsafe {
                let p = (*ptr).as_mut();
                // spin wait
                if p.is_some() { continue }
                std::ptr::write(ptr, Some(item));
                return;
            };
        };
    }
}

fn make_fixed_size<T>(buffer: Box<[T]>) -> Box<[T; QUEUE_CAPACITY as usize]> {
    assert_eq!(buffer.len(), QUEUE_CAPACITY as usize);

    // safety: We check that the length is correct.
    unsafe { Box::from_raw(Box::into_raw(buffer).cast()) }
}

fn is_full(head: u64, tail: u64) -> bool {
    tail.wrapping_sub(head) == QUEUE_CAPACITY
}

fn is_empty(head: u64, tail: u64) -> bool {
    tail == head
}

fn unpack(val: u32) -> (u16, u16) {
    let steal = (val & UNPACK_MASK) as u16;
    let head = (val >> 16) as u16;
    (head, steal)
}

fn pack(head: u16, steal: u16) -> u32 {
    (head as u32) << 16 | steal as u32
}

#[cfg(test)]
mod tests {
    use crate::u64_steal::{pack, unpack, Inner, Queue, Steal, QUEUE_CAPACITY};
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};
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
    fn test_pack_unpack() {
        let a = 10;
        let b = 20;
        let packed = pack(a, b);
        let (s_a, s_b) = unpack(packed);

        assert_eq!(a, s_a);
        assert_eq!(b, s_b);
    }

    #[test]
    fn test_queue() {
        let inner = Inner::new();
        let queue = Queue::<TestItem>(Arc::new(inner));

        queue.push(TestItem {}).unwrap();
        assert_eq!(queue.size(), 1);

        let i = queue.pop().unwrap();
        assert_eq!(i, TestItem {});
        assert!(queue.pop().is_none());
    }

    #[test]
    fn test_queue_full() {
        let inner = Inner::new();
        let queue = Queue::<TestItem>(Arc::new(inner));

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
        let queue = Queue::<TestItem>(Arc::new(inner));

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
        let inner = Arc::new(Inner::new());
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
        println!("u64 per loop: {:}ms", duration);

        let s = set.lock().unwrap();
        for n in 0..i {
            if !s.contains(&IncrItem{i: n}){
                panic!("n not found: {n:}");
            }
        }
    }
}
