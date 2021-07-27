struct Solution;

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1];k as usize + 1];

        for i in 1..n as usize + 1 {
            dp[1][i] = i as i32;
        }

        for i in 2..k as usize + 1 {
            dp[i][1] = 1;
            for j in 2.. n as usize + 1 {
                dp[i][j] = Self::find_binary(&dp, i, j);
            }
        }
        // println!("{:?}", dp);
        dp[k as usize][n as usize]
    }

    fn find_linear(dp: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        let mut r = j as i32;
        for k in 1..j+1 {
            r = std::cmp::min(
                r,
                std::cmp::max(
                    dp[i-1][k-1],
                    dp[i][j-k]
                ) + 1
            );
        }

        r
    }

    fn find_binary(dp: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        let mut lo = 1;
        let mut hi = j;
        let mut result = j as i32;
        while lo <= hi {
            let mid = (hi - lo) / 2 + lo;
            if dp[i-1][mid-1] == dp[i][j - mid] { return dp[i][j - mid] + 1; }
            if dp[i-1][mid-1] > dp[i][j - mid] {
                hi = mid - 1;
                result = std::cmp::min(result, dp[i - 1][mid - 1]);
            }
            else {
                lo = mid + 1;
                result = std::cmp::min(result, dp[i][j - mid]);
            }
        }
        result as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::super_egg_drop::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::super_egg_drop(2, 6), 3);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::super_egg_drop(3, 14), 4);
    }
}