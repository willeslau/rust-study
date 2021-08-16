struct KMP {
    pattern: Vec<char>,
    lsp: Vec<usize>,
}

impl KMP {
    pub fn new(pattern: Vec<char>) -> Self {
        KMP {
            lsp: KMP::derive_lsp(&pattern),
            pattern,
        }
    }

    fn derive_lsp(pattern: &Vec<char>) -> Vec<usize> {
        let mut lsp = vec![0; pattern.len()];
        let mut j = 0;
        let mut i = 1;
        while i < pattern.len() {
            if pattern[i] == pattern[j] {
                j += 1;
                lsp[i] = j;
                i += 1;
            } else if j == 0 {
                lsp[i] = 0;
                i += 1;
            } else {
                j = lsp[j-1];
            }
        }
        lsp
    }

    pub fn find_matches(&self, string: String) -> Vec<usize> {
        let mut indexes = vec![];
        let chars: Vec<char> = string.chars().collect();

        let mut j = 0;
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == self.pattern[j] {
                j += 1;

                // pattern found
                if j == self.pattern.len() {
                    indexes.push(i);
                    j = self.lsp[j-1];
                }

                i += 1;

            } else if j > 0 {
               j = self.lsp[j-1];
            } else {
                i += 1;
            }
        }

        indexes
    }
}

#[cfg(test)]
mod tests {
    use crate::string::kmp::KMP;

    #[test]
    fn test_lsp() {
        let v = vec!['a', 'b', 'b', 'c'];
        assert_eq!(KMP::derive_lsp(&v), vec![0,0,0,0]);

        let v = vec!['a', 'b', 'b', 'a'];
        assert_eq!(KMP::derive_lsp(&v), vec![0,0,0,1]);
        
        let v = vec!['a', 'b', 'b', 'a'];
        assert_eq!(KMP::derive_lsp(&v), vec![0,0,0,1]);

        let v = vec!['a', 'b', 'b', 'a', 'b', 'b', 'b', 'a', 'b', 'a','a','b'];
        assert_eq!(KMP::derive_lsp(&v), vec![0,0,0,1,2,3,0,1,2,1,1,2]);
    }

    #[test]
    fn test_case1() {
        let kmp = KMP::new("abb".chars().collect());
        assert_eq!(kmp.find_matches("abbabb".to_string()), vec![2,5]);
    }

    #[test]
    fn test_case2() {
        let kmp = KMP::new("bbb".chars().collect());
        assert_eq!(kmp.find_matches("abbbbb".to_string()), vec![3, 4, 5]);
    }
}
