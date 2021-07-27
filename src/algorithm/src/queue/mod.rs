use std::cmp::Ordering;

mod priority_queue;
mod key_value_queue;

pub struct SimplePriorityQueue<T: Ord + Default> {
    data: Vec<T>,
    size: usize,
    capacity: usize,
}

impl<T: Ord + Default> SimplePriorityQueue<T> {
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity+1);
        for _ in 0..capacity+1 { data.push(T::default()); }
        SimplePriorityQueue {
            data,
            size: 0,
            capacity,
        }
    }

    pub fn insert(&mut self, value: T) {
        // maybe throw an error, anyways, it's simple impl.
        if self.size == self.capacity { return; }

        self.size += 1;
        self.data[self.size] = value;
        self.swim(self.size);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 { return None; }

        self.swap(1, self.size);
        let d = std::mem::replace(&mut self.data[self.size], T::default());
        self.size -= 1;

        self.sink(1);

        Some(d)
    }

    /* ---- Internal Methods ---- */

    fn sink(&mut self, mut k: usize) {
        let n = k * 2;
        while k * 2 <= self.size {
            let mut j = k * 2;
            if j < self.size && self.less(j, j+1) { j += 1; }
            if self.less(k, j) { self.swap(k, j); }
            k = j;
        }
    }

    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.less(k / 2, k) {
            self.swap(k / 2, k);
            k = k / 2;
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        unsafe {
            let ptr_a: *mut T = &mut self.data[i];
            let ptr_b: *mut T = &mut self.data[j];
            std::ptr::swap(ptr_a, ptr_b);
        }
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.data[i].cmp(&self.data[j]).is_lt()
    }
}

/// Priority queue with more unsafe rust
pub struct PriorityQueue<T: Ord + Default> {
    data: Vec<T>,
    size: usize,
    capacity: usize,
}

impl<T: Ord + Default> PriorityQueue<T> {
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity+1);
        for _ in 0..capacity+1 { data.push(T::default()); }
        PriorityQueue {
            data,
            size: 0,
            capacity,
        }
    }

    pub fn insert(&mut self, value: T) {
        // maybe throw an error, anyways, it's simple impl.
        if self.size == self.capacity { return; }

        self.size += 1;
        self.data[self.size] = value;
        self.swim(self.size);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 { return None; }

        self.swap(1, self.size);
        let d = std::mem::replace(&mut self.data[self.size], T::default());
        self.size -= 1;

        self.sink(1);

        Some(d)
    }

    /* ---- Internal Methods ---- */

    fn sink(&mut self, mut k: usize) {
        let n = k * 2;
        while k * 2 <= self.size {
            let mut j = k * 2;
            if j < self.size && self.less(j, j+1) { j += 1; }
            if self.less(k, j) { self.swap(k, j); }
            k = j;
        }
    }

    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.less(k / 2, k) {
            self.swap(k / 2, k);
            k = k / 2;
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        unsafe {
            let ptr_a: *mut T = &mut self.data[i];
            let ptr_b: *mut T = &mut self.data[j];
            std::ptr::swap(ptr_a, ptr_b);
        }
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.data[i].cmp(&self.data[j]).is_lt()
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::SimplePriorityQueue;

    #[test]
    fn it_works() {
        let mut p = SimplePriorityQueue::new(10);
        p.insert(2);
        p.insert(1);
        p.insert(0);
        p.insert(3);

        assert_eq!(p.pop(), Some(3));
        assert_eq!(p.pop(), Some(2));
        assert_eq!(p.pop(), Some(1));
        assert_eq!(p.pop(), Some(0));
        assert_eq!(p.pop(), None);
    }
}