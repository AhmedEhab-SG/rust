use std::collections::HashMap;

#[allow(unused_imports)]
use utils::Runable;

use crate::Solution;

// problem: https://leetcode.com/problems/group-anagrams/description
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut key = [0u8; 26];

            for b in s.bytes() {
                key[(b - b'a') as usize] += 1;
            }

            result.entry(key).or_insert(Vec::new()).push(s);
        }

        result.into_values().collect()
    }
}

impl Runable for Solution {
    fn run() {
        assert_eq!(
            Self::group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ],),
            vec![
                vec!["tan".to_string(), "nat".to_string()],
                vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
                vec!["bat".to_string()]
            ]
        );

        assert_eq!(
            Self::group_anagrams(vec!["".to_string()]),
            vec![vec!["".to_string()]]
        );

        assert_eq!(
            Self::group_anagrams(vec!["a".to_string()]),
            vec![vec!["a".to_string()]]
        );
    }
}
