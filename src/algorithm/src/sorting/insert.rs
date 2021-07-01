use crate::Sorting;
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ptr;

#[derive(Debug)]
pub struct InsertionSort<T>{
	phantom: PhantomData<T>
}

impl <T> Sorting<T> for InsertionSort<T> where T: Ord {
	fn sort(data: &mut Vec<T>) {
		if data.len() < 2 { return; }

		for i in 1..data.len() {
			for j in (0..i).rev() {
				match data[j+1].cmp(&data[j]) {
					Ordering::Less => Self::swap(data, j, j+1),
					Ordering::Greater => break,
					_ => {}
				}
			}
		}
	}
}


impl <T> InsertionSort<T> where T: Ord {
	fn swap(data: &mut Vec<T>, i: usize, j: usize) {
		unsafe {
			let ptr_a: *mut T = &mut data[i];
			let ptr_b: *mut T = &mut data[j];
			ptr::swap(ptr_a, ptr_b);
		}
	}
}


#[cfg(test)]
mod tests {
	use crate::sorting::insert::InsertionSort;
	use crate::Sorting;

	#[test]
    fn it_works() {
        let mut v = vec![5,4,3,2,4,5,6,7];
        InsertionSort::sort(&mut v);

        assert_eq!(v, vec![2, 3, 4, 4, 5, 5, 6, 7]);        
    }


	#[test]
    fn already_sorted() {
        let mut v = vec![1,2,3,4,5,6,7];
        InsertionSort::sort(&mut v);

        assert_eq!(v, vec![1,2,3,4,5,6,7]);        
    }
}