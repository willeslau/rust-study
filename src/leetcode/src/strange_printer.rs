struct Solution {}

/// One dimensional dp is faster
// impl Solution {
//     pub fn strange_printer(s: String) -> i32 {
//         let s = s.as_bytes();
//         let n = s.len();
//         let mut memo = vec![0; n*n];
//         for lo in (0..n).rev() {
//             memo[lo*n+lo] = 1;
//             for hi in lo+1..n {
//                 if s[lo] == s[hi] {
//                     memo[hi*n+lo] = memo[hi*n+lo+1];
//                 } else {
//                     let mut res = i32::MAX;
//                     for k in lo..hi {
//                         res = res.min(memo[k*n+lo]+memo[hi*n+k+1]);
//                     }
//                     memo[hi*n+lo] = res;
//                 }
//             }
//         }
//         memo[(n-1)*n]
//     }
// }

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![chars.len();chars.len()];chars.len()];

        for i in 0..chars.len() { dp[i][i] = 1; }
        for size in 2..chars.len()+1 {
            for i in 0..chars.len()-size+1 {
                let j = i + size - 1;
                if chars[i] == chars[j] { dp[i][j] = dp[i][j-1]; }
                else {
                    for k in i..j {
                        dp[i][j] = std::cmp::min(dp[i][j], dp[i][k] + dp[k+1][j]);
                    }
                }
            }
        }
        // println!("{:?}", dp);

        dp[0][chars.len() - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::strange_printer::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::strange_printer("aaabbb".to_string()),
            2
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::strange_printer("aba".to_string()),
            2
        );
    }
}