use std::collections::HashMap;

struct Solution;


impl Solution {
	pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
		let mut map: HashMap<i32, i32> = HashMap::new();
		let mut result = vec![0;2];
		for i in 0..nums.len() {
			let v = target - nums[i];
			if map.contains_key(&v) {
				result[0] = *map.get(&v).unwrap();
				result[1] = i as i32;
				break;
			} else {
				map.insert(nums[i], i as i32);
			}
		}

		result
    }

    pub fn nested_hashmap(nums: Vec<i32>) {
		let mut nested: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

		for i in 0..nums.len() {
			if !nested.contains_key(&nums[i]) {
				nested.insert(nums[i], HashMap::new());
			} else {
				let m = nested.get_mut(&nums[i]).unwrap();
				m.insert(nums[i], 1);
			}
		}

		println!("{:?}", nested);
    }
}


#[cfg(test)]
mod tests {
    use crate::two_sum::Solution;

    #[test]
    fn it_works() {
    	let v = vec![2,7,11,15];
    	assert_eq!(Solution::two_sum(v, 9), vec![0,1]);
    }


    #[test]
    fn nested_hashmap_works() {
    	let v = vec![2,2,2,2,3,3,3,7,11,15];
    	Solution::nested_hashmap(v);
    }
}