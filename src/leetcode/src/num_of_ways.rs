struct Solution;

impl Solution {
    fn check_one(n: (i32, i32, i32)) -> bool {
        n.0 != n.1 && n.1 != n.2
    }

    fn check_pair(p: (i32, i32, i32), q: (i32, i32, i32)) -> bool {
        p.0 != q.0 && p.1 != q.1 && p.2 != q.2
    }

    pub fn num_of_ways(n: i32) -> i32 {
        let mod_num = 1000000007;
        let mut types = vec![];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    if Self::check_one((i, j, k)) {
                        types.push((i, j, k));
                    }
                }
            }
        }

        let n = n as usize;
        let mut dp = vec![0;types.len()];
        let mut aux = vec![0;types.len()];

        for i in 0..types.len() {
            dp[i] = 1;
        }

        for _ in 1..n {
            for i in 0..types.len() {
                let mut result = 0;
                for j in 0..types.len() {
                    if Self::check_pair(types[i], types[j]) {
                        result = (result + dp[j]) % mod_num;
                    }
                }
                aux[i] = result;
            }
            std::mem::swap(&mut aux, &mut dp);
        }

        let mut r = 0;
        for f in dp {
            r = (f + r) % mod_num;
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use crate::num_of_ways::Solution;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::num_of_ways(3),
            246
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::num_of_ways(2),
            54
        );
    }
}