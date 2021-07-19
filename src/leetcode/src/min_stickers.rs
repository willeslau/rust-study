struct Solution;

impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let target: Vec<char> = target.chars().collect();
        let stickers: Vec<Vec<char>> = stickers.iter().map(|a| a.chars().collect()).collect();

        let mut dp = vec![-1;1<<target.len()];
        dp[0] = 0;

        for state in 0..1 << target.len() {
            if dp[state] == -1 { continue; }
            for sticker in &stickers {
                let mut next_state = state;
                for ch in sticker {
                    for i in 0..target.len() {
                        if (next_state >> i) & 1 == 1 { continue; }
                        if *ch == target[i] {
                            next_state |= 1 << i;
                            break;
                        }
                    }
                }

                if dp[next_state] == -1 || dp[next_state] > dp[state] + 1 {
                    dp[next_state] = dp[state] + 1;
                }
            }
        }

        println!("{:?}", dp);

        dp[(1 << target.len()) - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::min_stickers::Solution;

    #[test]
    fn test_case1() {
        let v = vec!["with", "example", "science"].iter().map(|a| a.to_string()).collect();
        let target = "thehat".to_string();

        assert_eq!(
            Solution::min_stickers(v, target),
            3
        );
    }

    #[test]
    fn test_case2() {
        let v = vec!["notice", "possible"].iter().map(|a| a.to_string()).collect();
        let target = "basicbasic".to_string();

        assert_eq!(
            Solution::min_stickers(v, target),
            -1
        );
    }
}