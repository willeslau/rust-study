use std::cmp::{min, max};

struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut graph = vec![vec![0; n + 1]; n + 1];

        for v in &times {
            graph[v[0] as usize][v[1] as usize] = v[2];
        }
        // println!("{:?}", graph);

        let mut is_visited = vec![i32::MAX; n + 1];
        Self::dfs(&graph, k, 0, &mut is_visited);

        println!("{:?}", is_visited);
        let mut max_len = 0;
        for i in 1..is_visited.len() {
            if is_visited[i] == i32::MAX { return -1; }
            max_len = max(max_len, is_visited[i]);
        }
        max_len
    }

    fn dfs(graph: &Vec<Vec<i32>>, index: usize, dis: i32, is_visited: &mut Vec<i32>) {
        if is_visited[index] < dis { return; }
        is_visited[index] = min(i32::MAX, dis);

        let candidates = &graph[index];
        for i in 1..candidates.len() {
            if candidates[i] != 0 {
                Self::dfs(graph, i, dis + candidates[i], is_visited);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::network_delay_time::Solution;

    #[test]
    fn test_case_1() {
        let times = [[2, 1, 1], [2, 3, 1], [3, 4, 1]].map(|a| a.to_vec()).to_vec();
        let n = 4;
        let k = 2;
        assert_eq!(
            Solution::network_delay_time(times, n, k),
            2
        );
    }

    #[test]
    fn test_case_2() {
        let times = [[1, 2, 1]].map(|a| a.to_vec()).to_vec();
        let n = 2;
        let k = 1;
        assert_eq!(
            Solution::network_delay_time(times, n, k),
            1
        );
    }

    #[test]
    fn test_case_3() {
        let times = [[1, 2, 1]].map(|a| a.to_vec()).to_vec();
        let n = 2;
        let k = 2;
        assert_eq!(
            Solution::network_delay_time(times, n, k),
            -1
        );
    }

    #[test]
    fn test_case_4() {
        let times = [[1, 2, 1], [2, 1, 3]].map(|a| a.to_vec()).to_vec();
        let n = 2;
        let k = 2;
        assert_eq!(
            Solution::network_delay_time(times, n, k),
            3
        );
    }

    #[test]
    fn test_case_5() {
        let times = [[1,2,1],[2,3,2],[1,3,2]].map(|a| a.to_vec()).to_vec();
        let n = 3;
        let k = 1;
        assert_eq!(
            Solution::network_delay_time(times, n, k),
            2
        );
    }

    #[test]
    fn test_case_5() {
        let times = [[1,2,1],[2,3,2],[1,3,2]].map(|a| a.to_vec()).to_vec();
        let n = 3;
        let k = 1;
        assert_eq!(
            Solution::network_delay_time(times, n, k),
            2
        );
    }
}
