#[allow(unused_imports)]
use utils::Runable;

use crate::Solution;

// problem: https://leetcode.com/problems/valid-anagram/description/
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        true
    }
}

impl Runable for Solution {
    fn run() {
        assert_eq!(
            Self::is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );

        assert_eq!(
            Self::is_anagram("rat".to_string(), "car".to_string()),
            false
        );
    }
}
