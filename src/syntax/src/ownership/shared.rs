/// This is about shared ownership. We will cover Rc, Cell and RefCell.
/// Some good readings are:
/// https://badboi.dev/rust/2020/07/17/cell-refcell.html

use std::rc::Rc;
use std::cell::RefCell;

fn rc_length(s: Rc<String>) -> usize {
    s.len()
}

fn length(s: String) -> usize {
    s.len()
}

fn min_sum_refcell(min: i32, v: &RefCell<Vec<i32>>) {
    let sum: i32 = v.borrow().iter().sum();
    if sum < min {
        v.borrow_mut().push(min - sum);
    }
}

struct Data<'a> {
    d: &'a mut usize,
}

impl Data<'_> {
    pub fn increment(&mut self) {
        *self.d += 1;
    }
}

struct Data2<'a> {
    d: &'a Data<'a>,
}

impl Data2<'_> {
    pub fn get(&self) -> usize {
        // here will have a `Copy` invoked
        *self.d.d + 1
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use crate::ownership::shared::{rc_length, length, Data, Data2};
    use std::rc::Rc;
    use test::Bencher;
    use std::cell::{RefCell, RefMut};
    use std::collections::HashMap;

    #[bench]
    fn rc_length_works(b: &mut Bencher) {
        let v: Rc<String> = Rc::new((0..100_000).map(|_| 'a').collect());
        b.iter(|| {
            test::black_box(rc_length(v.clone()));
        });
        // assert_eq!(rc_length(v), 100_000_000);
    }

    #[bench]
    fn length_works(b: &mut Bencher) {
        let v: String = (0..100_000).map(|_| 'a').collect();
        b.iter(|| {
            test::black_box(length(v.clone()));
        });
    }

    #[test]
    fn it_works() {
        let mut v: usize = 0;
        let mut k = &Data{ d: &mut v };
        let data = Data2{ d: k };
        let f = data.get();
        v += 1;
        // assert_eq!(v, f.d);
    }

    #[test]
    fn test_with_hashmap() {
        let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
        // Create a new block to limit the scope of the dynamic borrow
        {
            let mut map: RefMut<_> = shared_map.borrow_mut();
            map.insert("africa", 92388);
            map.insert("kyoto", 11837);
            map.insert("piccadilly", 11826);
            map.insert("marbles", 38);
        }

        // Note that if we had not let the previous borrow of the cache fall out
        // of scope then the subsequent borrow would cause a dynamic thread panic.
        // This is the major hazard of using `RefCell`.
        let total: i32 = shared_map.borrow().values().sum();
        println!("{}", total);
    }
}