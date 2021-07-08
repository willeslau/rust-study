struct Solution;

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
    	let mut dp = vec![vec![0;3];nums.len()];

    	let mut sum: i32 = 0;
    	let k = k as usize;
    	let mut result = vec![0,0,0];

    	for i in 0..k { sum += nums[i]; }
    	dp[k-1][0] = sum;

    	for i in k..nums.len() {    		
    		sum += nums[i] - nums[i-k];
    		dp[i][0] = sum;
    	}

    	let mut index = vec![0;nums.len()];
    	for i in k..nums.len() {
    		for j in k-1..i-k+1 {
    			let v = dp[i][0] + dp[j][0];
    			if v > dp[i][1] {
    				dp[i][1] = v;
    				index[i] = j;
    			}
    		}
    	}

    	let f = 2*k;
		let mut max = 0;
    	for i in f-1..nums.len() {
    		for j in f-1..i+1-k {
    			let v = dp[i][0] + dp[j][1];
    			if v > max {
    				result[0] = (index[j] +1 - k) as i32 ;
    				result[1] = (j - k + 1) as i32;
    				result[2] = (i - k + 1) as i32;
    				max = v;
    			}
    		}
    	}

    	return result;
    }
}


#[cfg(test)]
mod tests {
	use crate::max_sum_of_three_subarrays::Solution;

	#[test]
	fn it_works() {
		let v = vec![1,2,1,2,6,7,5,1];
		let r = Solution::max_sum_of_three_subarrays(v, 2);
	}
}