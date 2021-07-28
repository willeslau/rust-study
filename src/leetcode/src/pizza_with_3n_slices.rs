use std::cmp::{ min, max };
struct Solution;


impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        if slices.len() <= 3 {
            return *slices.iter().min().unwrap();
        }

        let mut is_visited = vec![false; slices.len()];
        let mut cache = vec![0; slices.len()];

        // handle index 0
        is_visited[0] = true;
        is_visited[slices.len()-1] = true;
        is_visited[1] = true;
        let mut max_cake =
            slices[0]
            + slices[1]
            + slices[slices.len()-1]
            + Self::slices(&slices, 2, slices.len()-1, &mut is_visited, &mut cache);

        is_visited[0] = false;
        is_visited[slices.len()-1] = false;
        is_visited[1] = false;

        // main loop
        for i in 1..slices.len() {
            is_visited[i-1] = true;
            is_visited[i] = true;
            is_visited[i+1] = true;

            max_cake = max(
                max_cake,
                slices[i-1]
                    + slices[i]
                    + slices[i+1]
                    + Self::slices(&slices, i+2, i-2+slices.len(), &mut is_visited, &mut cache)
            );

            is_visited[i-1] = false;
            is_visited[i] = false;
            is_visited[i+1] = false;

        }

        max_cake
    }

    fn deduce_segments(is_visited: &Vec<bool>) -> Vec<(usize, usize)> {
        vec![]
    }

    fn slices(slices: &Vec<i32>, lo: usize, hi: usize, is_visited: &mut Vec<bool>, cache: &mut Vec<i32>) -> i32 {
        let index = Self::deduce_index(lo, hi);
        if cache[index] != 0 { return cache[index]; }

        let cake_num = Self::get_cake_num(lo, hi, &is_visited);
        if cake_num < 3 {
            cache[index] = 0;
            return 0;
        }

        let mut max_cake_size = 0;
        for i in lo..hi+1 {
            if is_visited[i] { continue; }
            let prev = Self::deduce_next(i, is_visited);
            let next = Self::deduce_prev(i, is_visited);

            is_visited[i] = true;
            is_visited[prev] = true;
            is_visited[next] = true;

            let mut cake_size = slices[i] + slices[prev] + slices[next];
            for (i, j) in Self::deduce_segments(is_visited) {
                cake_size += Self::slices(&slices, i, j, is_visited, cache);
            }

            max_cake_size = max(max_cake_size, cake_size);
        }
        cache[index] = max_cake_size;

        max_cake_size
    }
}