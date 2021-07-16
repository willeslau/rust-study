struct Solution;

impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let mut hi = 0;
        let mut lo = 0;
        for n in &jobs {
            if *n > lo { lo = *n; }
            hi += n;
        }

        let mut jobs = jobs;
        jobs.sort();

        println!("lo: {}, hi: {}", lo, hi);

        while lo <= hi {
            let mut mid = (hi - lo) / 2 + lo;
            let mut visited = vec![false;jobs.len()];
            if Self::within_limit(&jobs, mid, 0, 0, k, 0, &mut visited) {
                println!("lo: {}, mid: {}, hi: {}, within: true", lo, mid, hi);
                hi = mid - 1;
            } else {
                println!("lo: {}, mid: {}, hi: {}, within: false", lo, mid, hi);
                lo = mid + 1;
            }
        }

        lo
    }

    fn within_limit(jobs: &Vec<i32>, limit: i32, sum_so_far: i32, count_so_far: usize, k: i32, begin: usize, visited: &mut Vec<bool>) -> bool {
        if sum_so_far > limit {
            println!("here");
            return false;
        }
        if count_so_far == jobs.len() {
            return true;
        }
        if k == 0 {
            println!("limit {}, count: {}", limit, count_so_far);
            return count_so_far == jobs.len();
        }

        for i in begin..jobs.len() {
            if visited[i] { continue; }

            let n = sum_so_far + jobs[i];
            if n > limit {
                return Self::within_limit(jobs, limit, 0, count_so_far, k - 1, 0, visited);
            } else if n == limit {
                visited[i] = true;
                let r = Self::within_limit(jobs, limit, 0, count_so_far+1, k - 1, 0, visited);
                visited[i] = false;
                return r;
            } else {
                visited[i] = true;
                if Self::within_limit(jobs, limit, sum_so_far + jobs[i], count_so_far+1, k, begin+1, visited) {
                    return true;
                }
                visited[i] = false;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::minimum_time_required::Solution;

    #[test]
    fn test_case_1() {
        let jobs = [3, 2, 3];
        let k = 3;
        assert_eq!(
            Solution::minimum_time_required(Vec::from(jobs), k),
            3
        );
    }

    #[test]
    fn test_case_2() {
        let jobs = [1, 2, 4, 7, 8];
        let k = 2;
        assert_eq!(
            Solution::minimum_time_required(Vec::from(jobs), k),
            11
        );
    }
}
