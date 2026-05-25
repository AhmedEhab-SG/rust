use std::{char, collections::HashSet};

#[allow(unused_imports)]
use utils::Runable;

use crate::Solution;

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<_> = s.chars().collect();

        let mut seen: HashSet<char> = HashSet::new();

        let mut left = 0;
        let mut longest = 0;

        for right in 0..chars.len() {
            while seen.contains(&chars[right]) {
                seen.remove(&chars[left]);
                left += 1;
            }

            seen.insert(chars[right]);

            longest = longest.max(right - left + 1);
        }

        longest as i32
    }
}

// impl Runable for Solution {
//     fn run() {
//         assert_eq!(
//             Solution::length_of_longest_substring(String::from("abcabcbb")),
//             3
//         );
//
//         assert_eq!(
//             Solution::length_of_longest_substring(String::from("bbbbb")),
//             1
//         );
//
//         assert_eq!(
//             Solution::length_of_longest_substring(String::from("pwwkew")),
//             3
//         );
//     }
// }
