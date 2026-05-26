#[allow(unused_imports)]
use utils::Runable;

use crate::Solution;

// problem: https://leetcode.com/problems/two-sum/
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        vec![]
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
                ["bat".to_string()],
                ["nat".to_string(), "tan".to_string()],
                ["ate".to_string(), "eat".to_string(), "tea".to_string()]
            ]
        );

        assert_eq!(Self::group_anagrams(vec![""], 6), vec![1, 2]);

        assert_eq!(Self::group_anagrams(vec!["a"], 6), vec![0, 1]);
    }
}
