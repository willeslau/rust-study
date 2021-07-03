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


#[cfg(test)]
mod tests {
    extern crate test;
    use crate::ownership::shared::{rc_length, length};
    use std::rc::Rc;
    use test::Bencher;
    use std::cell::RefCell;

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
    fn test_ref_cell() {
        let ref_cell = RefCell::new(vec![1,2,3,4,5]);

    }
}