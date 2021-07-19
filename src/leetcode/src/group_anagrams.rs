use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        let mut result = vec![];

        for s in strs {
            let key = Self::derive_key(s.clone());
            let index = map.get(&key);
            if index.is_none() {
                result.push(vec![s.clone()]);
                map.insert(key.clone(), result.len() - 1);
            } else {
                result[*index.unwrap()].push(s.clone());
            }
        }
        result
    }

    fn derive_key(s: String) -> String {
        let mut v: Vec<char> = s.chars().collect();
        v.sort();
        v.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::group_anagrams::Solution;

    #[test]
    fn test_case1() {
        let v = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        assert_eq!(
            Solution::group_anagrams(v),
            vec![
                vec!["ate", "eat", "tea"],
                vec!["nat", "tan"],
                vec!["bat"]
            ]
        );
    }
}