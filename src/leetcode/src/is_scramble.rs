struct Solution;

/// Interestingly, memorization seems to be faster than using DP
impl Solution {
	pub fn is_scramble_brutal(s1: String, s2: String) -> bool {
		if s1.len()  != s2.len() { return false; }

		let v1: Vec<char> = s1.chars().collect();
		let v2: Vec<char> = s2.chars().collect();

		Self::is_scramble_brutal_inner(&v1, 0, v1.len()-1, &v2, 0, v2.len()-1)
	}

	pub fn is_scramble_memorization(s1: String, s2: String) -> bool {
		if s1.len()  != s2.len() { return false; }

		let v1: Vec<char> = s1.chars().collect();
		let v2: Vec<char> = s2.chars().collect();

		let mut cache = vec![vec![vec![0;s1.len()];s1.len()];s1.len()];
		Self::is_scramble_memorization_inner(&v1, 0, v1.len()-1, &v2, 0, v2.len()-1, &mut cache)
		
	}

	pub fn is_scramble_dp(s1: String, s2: String) -> bool {
		if s1.len()  != s2.len() { return false; }

		let v1: Vec<char> = s1.chars().collect();
		let v2: Vec<char> = s2.chars().collect();

		let mut dp = vec![vec![vec![false;s1.len()+1];s1.len()];s1.len()];

		// process dp[i][j][0] first
		for i in 0..s1.len() {
			for j in 0..s1.len() {
				dp[i][j][1] = v1[i] == v2[j];
			}
		}
		// Self::print_k(&dp, 1);

		for k in 2..s1.len()+1 {
			for i in 0..s1.len()-k+1 {
				if i + k > s1.len() { break; }

				for j in 0..s1.len() {
					if j + k > s1.len() { break; }

					// process dp[i][j][1..s1.len()]
					// dp[i][j][k] = is_scramble(dp[i][j][1], dp[i+1][j+1][k-1]) || is_scramble(dp[i][j][k-1], dp[i+k-1][j+k-1][1]) ||
					//               is_scramble(dp[i][j][2], dp[i+1][j+1][k-2]) || is_scramble(dp[i][j][k-2], dp[i+k-2][j+k-2][2])
					for h in 1..k {
						dp[i][j][k] = (dp[i][j][h] && dp[i+h][j+h][k-h]) || (dp[i][j+k-h][h] && dp[i+h][j][k-h]);
						if dp[i][j][k] { break; }
					}
				}
			}
		}
		

		dp[0][0][s1.len()]
	}

	fn print_k(v: &Vec<Vec<Vec<bool>>>, k: usize) {
		for i in 0..v.len() {
			for j in 0..v[0].len() {
				println!("i: {}, j: {}, dp[{}][{}][{}]: {}", i, j, i, j, k, v[i][j][k]);
			}
		}
		println!("");
	}

	/// caller should have ensured j1 - i1 == j2 - i2
	fn is_same_char_count(v1: &Vec<char>, i1: usize, j1: usize, v2: &Vec<char>, i2: usize) -> bool {
		let mut v1_count = vec![0;26];
		let mut v2_count = vec![0;26];

		for i in 0..(j1 - i1 + 1) {
			v1_count[v1[i1+i] as usize - 'a' as usize] += 1;
			v2_count[v2[i2+i] as usize - 'a' as usize] += 1;
		}

		for i in 0..26 {
			if v2_count[i] != v1_count[i] { return false; }
		}

		true
	}

	fn is_equal(v1: &Vec<char>, i1: usize, j1: usize, v2: &Vec<char>, i2: usize) -> bool {
		for i in 0..(j1 - i1 + 1) {
			if v1[i1+i] != v2[i2+i] { return false; }
		}
		true
	}

	fn is_scramble_memorization_inner(v1: &Vec<char>, i1: usize, j1: usize, v2: &Vec<char>, i2: usize, j2: usize, cache: &mut Vec<Vec<Vec<i32>>>) -> bool {
		if cache[i1][i2][j1-i1] != 0 { 
			// println!("cache hit");
			return cache[i1][i2][j1-i1] == 1;
		}

		if Self::is_equal(v1, i1, j1, v2, i2) { 
			cache[i1][i2][j1-i1] = 1;
			return true;
		}
		if !Self::is_same_char_count(v1, i1, j1, v2, i2) { 
			cache[i1][i2][j1-i1] = -1;
			return false;
		}

		for i in 0..j1-i1 {
			if Self::is_scramble_memorization_inner(v1, i1, i1+i, v2, i2, i2+i, cache) && 
			   Self::is_scramble_memorization_inner(v1, i1+i+1, j1, v2, i2+i+1, j2, cache) { 
			   	cache[i1][i2][j1-i1] = 1;
			   	return true;
			}

			if Self::is_scramble_memorization_inner(v1, i1, i1+i, v2, j2-i, j2, cache) &&
			   Self::is_scramble_memorization_inner(v1, i1+i+1, j1, v2, i2, j2-i-1, cache) { 
			   	cache[i1][i2][j1-i1] = 1;
			   	return true;
			}
		}

		cache[i1][i2][j1-i1] = -1;
		false
	}

	fn is_scramble_brutal_inner(v1: &Vec<char>, i1: usize, j1: usize, v2: &Vec<char>, i2: usize, j2: usize) -> bool {
		if Self::is_equal(v1, i1, j1, v2, i2) { return true; }
		if !Self::is_same_char_count(v1, i1, j1, v2, i2) { return false; }

		for i in 0..j1-i1 {
			if Self::is_scramble_brutal_inner(v1, i1, i1+i, v2, i2, i2+i) && 
			   Self::is_scramble_brutal_inner(v1, i1+i+1, j1, v2, i2+i+1, j2) { return true; }

			if Self::is_scramble_brutal_inner(v1, i1, i1+i, v2, j2-i, j2) &&
			   Self::is_scramble_brutal_inner(v1, i1+i+1, j1, v2, i2, j2-i-1) { return true; }
		}
		false
	}
}

#[cfg(test)]
mod tests {
	use crate::is_scramble::Solution;

	#[test]
	fn test_case1() {
		let s1 = "great".to_string();
		let s2 = "rgeat".to_string();

		assert_eq!(Solution::is_scramble_brutal(s1.clone(), s2.clone()), true);
		assert_eq!(Solution::is_scramble_memorization(s1.clone(), s2.clone()), true);
		assert_eq!(Solution::is_scramble_dp(s1, s2), true);
	}


	#[test]
	fn test_case2() {
		let s1 = "abcde".to_string();
		let s2 = "caebd".to_string();

		assert_eq!(Solution::is_scramble_brutal(s1.clone(), s2.clone()), false);
		assert_eq!(Solution::is_scramble_memorization(s1.clone(), s2.clone()), false);
		assert_eq!(Solution::is_scramble_dp(s1, s2), false);
	}
}