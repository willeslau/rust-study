use std::cmp::Ordering;


struct Solution {}

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
    	let mut v = envelopes;
    	v.sort_by(|a, b| {
    		match a[0].cmp(&b[0]) {
    			Ordering::Equal => a[1].cmp(&b[1]),
    			other => other,
    		}
    	});

    	let mut max = 1;
    	let mut dp = vec![1;v.len()];
    	for i in 1..v.len() {
    		for j in 0..i {
    			if Self::fit(&v[j], &v[i]) {
    				dp[i] = std::cmp::max(dp[j] + 1, dp[i]);
    			}
    		}
    		max = std::cmp::max(dp[i], max);
    	}

    	max
    }

    fn fit(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    	a[0] < b[0] && a[1] < b[1]
    }
}


#[cfg(test)]
mod tests {
    use crate::russian_doll_envelopes::Solution;

    #[test]
    fn it_works() {
    	let v = vec![vec![5,4], vec![6,4], vec![6,7], vec![2,3]];
    	assert_eq!(Solution::max_envelopes(v), 3);
    }
}