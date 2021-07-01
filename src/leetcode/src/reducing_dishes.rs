struct Solution;


impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
    	let mut dp = satisfaction.clone();
    	let mut max = 0;

    	dp.sort_by(|a, b| { b.cmp(&a) });

    	// println!("{:?}", dp);

    	for i in 1..satisfaction.len() {
    		let mut b = 0;
    		// here can be improved, use pre_sum
    		for j in 0..i+1 {
    			b += (j as i32 +1) * dp[i-j];
    		}

    		// println!("max: {} for {:?}", b, i);
    		max = std::cmp::max(max, b);
    	}

    	max
    }
}


#[cfg(test)]
mod tests {
	use crate::reducing_dishes::Solution;

	// #[test]
	// pub fn case_1() {
	// 	let v = vec![-1,-8,0,5,-9];
	// 	assert_eq!(Solution::max_satisfaction(v), 14);
	// }

	#[test]
	pub fn case_2() {
		let v = vec![-2,5,-1,0,3,-3];
		assert_eq!(Solution::max_satisfaction(v), 35);
	}

	
}