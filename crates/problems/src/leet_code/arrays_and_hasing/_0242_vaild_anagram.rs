use std::collections::HashMap;

#[allow(unused_imports)]
use utils::Runable;

use crate::Solution;

// problem: https://leetcode.com/problems/valid-anagram/description/
impl Solution {
    fn count_from_char(hash_s: &mut HashMap<char, usize>, s: String) {
        for c in s.chars() {
            *hash_s.entry(c).or_insert(1) += 1
        }
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        let mut hash_s: HashMap<char, usize> = HashMap::new();
        let mut hash_t: HashMap<char, usize> = HashMap::new();

        Self::count_from_char(&mut hash_s, s);
        Self::count_from_char(&mut hash_t, t);

        hash_s == hash_t
    }
}

// impl Runable for Solution {
//     fn run() {
//         assert_eq!(
//             Self::is_anagram("anagram".to_string(), "nagaram".to_string()),
//             true
//         );
//
//         assert_eq!(
//             Self::is_anagram("rat".to_string(), "car".to_string()),
//             false
//         );
//     }
// }
