use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::ops::{Deref, DerefMut};

struct Solution;

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();

        for w in words.clone() {
            trie.insert(w);
        }

        let mut results = vec![];
        for w in words {
            if trie.is_concat_words(&*trie.root.borrow(), &w.chars().collect(), 0, 0) {
                results.push(w);
            }
        }
        results
    }
}

struct Trie {
    root: RefCell<Node>,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: RefCell::new(Node { symbol: ' ', value: None, children: None }) }
    }

    pub fn insert(&self, item: String) {
        let chars = item.chars().collect();
        self.insert_inner(&mut *self.root.borrow_mut(), &chars, 0);
    }

    fn contains(node: &Node, chars: &Vec<char>, index: usize) -> bool {
        let c = chars.get(index).unwrap();
        if index >= chars.len()
            || node.get_child(c).is_none() { return false; }

        let node = node.get_child(c).unwrap();
        if index == chars.len() - 1 && node.symbol == *c {
            return node.value.is_some();
        }

        return Self::contains(node, chars, index + 1);
    }

    pub fn is_concat_words(&self, node: &Node, chars: &Vec<char>, index: usize, count: usize) -> bool {
        if index >= chars.len() { return false; }
        let c = chars.get(index).unwrap();
        if let Some(n) = node.get_child(&c) {
            if n.value.is_some() {
                println!("value: {:?}, chars: {:?}, index:{}, count: {}", n.value, chars, index, count);
                if count > 0 && index == chars.len() - 1 { return true; }
                if self.is_concat_words(self.root.borrow().deref(), chars, index + 1, count + 1) {
                    return true;
                }
            }
            return self.is_concat_words(n, chars, index + 1, count);
        } else {
            false
        }
    }

    fn insert_inner(&self, node: &mut Node, chars: &Vec<char>, index: usize) {
        if index >= chars.len() { return; }
        let c = *chars.get(index).unwrap();

        if node.get_child(&c).is_none() {
            let n = Node::new(c);
            node.add_child(n);
        }

        let mut node = node.get_mut_child(&c).unwrap();
        if index == chars.len() - 1 {
            node.value = Some(chars.clone().iter().collect());
            return;
        }
        self.insert_inner(node, chars, index + 1);
    }
}

#[derive(Debug)]
struct Node {
    symbol: char,
    value: Option<String>,
    children: Option<HashMap<char, Box<Node>>>,
}

impl Node {
    fn new(symbol: char) -> Self {
        Node { symbol, value: None, children: None }
    }

    fn get_mut_child(&mut self, symbol: &char) -> Option<&mut Node> {
        match self.children.as_mut() {
            None => None,
            Some(mut children) => children.get_mut(symbol).map(|a| a.deref_mut())
        }
    }

    fn get_child(&self, symbol: &char) -> Option<&Node> {
        match self.children.as_ref() {
            None => None,
            Some(mut children) => children.get(symbol).map(|a| a.deref())
        }
    }

    fn add_child(&mut self, node: Node) {
        match self.children.as_mut() {
            None => {
                let mut m = HashMap::new();
                m.insert(node.symbol, Box::new(node));
                self.children = Some(m);
            }
            Some(mut map) => {
                if map.contains_key(&node.symbol) { return; }
                map.insert(node.symbol, Box::new(node));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::find_all_concatenated_words_in_a_dict::Solution;

    #[test]
    fn test_case_1() {
        let v = vec![
            "cat",
            "cats",
            "catsdogcats",
            "dog",
            "dogcatsdog",
            "hippopotamuses",
            "rat",
            "ratcatdogcat"
        ].iter().map(|a| a.to_string()).collect::<Vec<String>>();

        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(v),
            vec!["catsdogcats", "dogcatsdog", "ratcatdogcat"].iter().map(|a| a.to_string()).collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_case_2() {
        let v = vec!["cat", "dog", "catdog"].iter().map(|a| a.to_string()).collect::<Vec<String>>();

        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(v),
            vec!["catdog"].iter().map(|a| a.to_string()).collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_case_3() {
        let v = vec!["cat", "cats", "dog", "catdogs", "s", "ca", "t", "d", "g"].iter().map(|a| a.to_string()).collect::<Vec<String>>();

        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(v),
            vec!["cat", "cats", "catdogs"].iter().map(|a| a.to_string()).collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_case_4() {
        let v = vec!["", "d", "g"].iter().map(|a| a.to_string()).collect::<Vec<String>>();

        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(v),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_case_5() {
        let v = vec!["", "cats", "dog", "catdogs", "s", "ca", "t", "d", "g"].iter().map(|a| a.to_string()).collect::<Vec<String>>();

        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(v),
            vec!["cats", "catdogs"].iter().map(|a| a.to_string()).collect::<Vec<String>>()
        );
    }
}