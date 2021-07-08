struct Solution;


impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    	let row = matrix.len();
    	let col = matrix[0].len();

    	let mut prefix_sum = vec![vec![0;col+1];row+1];
    	for i in 0..matrix.len() {
    		for j in 0..matrix[0].len() {
    			prefix_sum[i+1][j+1] = prefix_sum[i][j+1] 
    				+ prefix_sum[i+1][j] + matrix[i][j] - prefix_sum[i][j];
    		}
    	}

    	// the main loop
    	let mut max_sum = i32::MIN;
    	for top in 0..row {
    		for bottom in top..row {
    			
    			let mut s = vec![0];
    			for right in 0..col {
    				let right_area = prefix_sum[bottom+1][right+1] - prefix_sum[top][right+1];

    				let idx = s.binary_search(&(right_area - k)).unwrap_or_else(|x| x);
					
					if idx < s.len() {
						max_sum = std::cmp::max(right_area - s[idx], max_sum);
					}
    				

					let idx = s.binary_search(&right_area).unwrap_or_else(|x| x);
					s.insert(idx, right_area);
    			}
    		}
    	}

    	max_sum

    }
}

#[cfg(test)]
mod tests {
	use crate::max_sum_submatrix::Solution;

	#[test]
	fn it_works() {
		let v = vec![vec![1,0,1], vec![0,-2,3]];
		assert_eq!(Solution::max_sum_submatrix(v, 2), 2);
	}


	#[test]
	fn test_case2() {
		let v = vec![vec![2,2,-1]];
		assert_eq!(Solution::max_sum_submatrix(v, 0), -1);
	}
}