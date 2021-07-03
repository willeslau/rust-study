use std::ptr;
use std::marker::PhantomData;
use std::cmp::{Ord, Ordering};
use std::fmt::Debug;

use crate::Sorting;

use rand::thread_rng;
use rand::seq::SliceRandom;


pub struct QuickSort<T: Ord + Debug> {
	phantom: PhantomData<T>
}

impl <T> Sorting<T> for QuickSort<T> where T: Ord + Debug {
	fn sort(data: &mut Vec<T>) {
		data.shuffle(&mut thread_rng());
		Self::quick_sort(data, 0, data.len()-1);
	}
}

impl <T> QuickSort<T> where T: Ord + Debug {
	fn partition(data: &mut Vec<T>, i: usize, j: usize) -> usize {
		if i >= j { return i; }

		let mut lo = i + 1;
		let mut hi = j;
		while lo <= hi {
			match data[lo].cmp(&data[i]) {
				Ordering::Less => { 
					lo += 1;
					continue;
				},
				_ => {},
			}


			match data[hi].cmp(&data[i]) {
				Ordering::Greater | Ordering::Equal => { 
					hi -= 1;
					continue;
				},
				_ => {},
			}
			
			Self::swap(data, lo, hi);
		}
		Self::swap(data, hi, i);

		hi
		
	}

	fn quick_sort(data: &mut Vec<T>, lo: usize, hi: usize) {
		if lo >= hi { return; }

		let i = Self::partition(data, lo, hi);
		Self::quick_sort(data, lo, i);
		Self::quick_sort(data, i+1, hi);

	}

	fn swap(data: &mut Vec<T>, lo: usize, hi: usize) {
		unsafe {
			let ptr_a: *mut T = &mut data[lo];
			let ptr_b: *mut T = &mut data[hi];

			ptr::swap(ptr_a, ptr_b);
		}
	}

}

#[cfg(test)]
mod tests {
	use crate::Sorting;
    use crate::sorting::quick::QuickSort;
    use rand::Rng;
    use std::time::Instant;
    use std::cmp::{Ordering};

    #[test]
    fn it_works() {
        let mut v = vec![5,4,3,2,4,5,6,7];
        QuickSort::sort(&mut v);
        assert_eq!(v, vec![2, 3, 4, 4, 5, 5, 6, 7]);        
    }

    #[test]
    fn speed() {
        let mut rng = rand::thread_rng();
        let v: Vec<u64> = (0..100000).map(|_| rng.gen_range(0, 1000)).collect();

        let mut f = v.clone();
        let start = Instant::now();
        QuickSort::sort(&mut f);
        let elapsed = start.elapsed();
        ensure_sorted(&f);
        println!("Millis: {} ms", elapsed.as_millis());

        let mut f = v.clone();
        let start = Instant::now();
        f.sort();
        let elapsed = start.elapsed();
        println!("Millis: {} ms", elapsed.as_millis());
    }

    fn ensure_sorted(data: &Vec<u64>) {
    	for i in 1..data.len() {
    		match data[i].cmp(&data[i-1]) {
    			Ordering::Less => panic!("not sorted!"),
    			_ => {}
    		}
    	}
    }
}