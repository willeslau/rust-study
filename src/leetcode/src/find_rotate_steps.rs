struct Solution;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut dp = vec![vec![i32::MAX;ring.len()];key.len()];
        let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];

        let ring_chars: Vec<char> = ring.chars().collect();
        for i in 0..ring_chars.len() {
            let char = ring_chars[i];
            let index = char as u8 - 'a' as u8;
            pos[index as usize].push(i);
        }

        let key_chars: Vec<char> = key.chars().collect();
        let char = key_chars[0];
        let index = char as usize - 'a' as usize;
        for i in &pos[index] {
            dp[0][*i] = std::cmp::min((0 - *i as i32).abs(), (ring.len() - *i) as i32) + 1;
        }

        println!("{:?}", dp);

        for i in 1..key_chars.len() {
            let char = key_chars[i-1];
            let index = char as usize - 'a' as usize;

            for j in &pos[key_chars[i] as usize - 'a' as usize] {
                for p in &pos[index] {
                    let k = (*j as i32 - *p as i32).abs();
                    let m = std::cmp::min(k, ring.len() as i32 - k);
                    dp[i][*j] = std::cmp::min(dp[i][*j], dp[i-1][*p] + m + 1);
                }
            }
        }
        println!("{:?}", dp);

        let mut min = i32::MAX;
        for i in &dp[key.len()-1] {
            min = std::cmp::min(min, *i);
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use crate::find_rotate_steps::Solution;

    #[test]
    fn test_case1() {
        let v = Solution::find_rotate_steps("godding".to_string(), "gd".to_string());
        assert_eq!(v, 4);
    }

    #[test]
    fn test_case2() {
        let v = Solution::find_rotate_steps("abcde".to_string(), "ade".to_string());
        assert_eq!(v, 6);
    }
}