#[allow(unused_imports)]
use utils::Runable;

use crate::Solution;

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
impl Solution {
    fn seek_palindrome(mut left: usize, mut right: usize, s: &str, len: usize, res: &mut String) {
        while right < len && s.chars().nth(left) == s.chars().nth(right) {
            if right - left + 1 > res.len() {
                *res = s[left..right + 1].to_string();
            }

            if left == 0 {
                break;
            }

            left -= 1;
            right += 1;
        }
    }

    pub fn longest_palindrome(s: String) -> String {
        let mut res = "".to_string();
        let len = s.len();

        for i in 0..len {
            // odd length
            Self::seek_palindrome(i, i, &s, len, &mut res);

            // even length
            if i + 1 < len {
                Self::seek_palindrome(i, i + 1, &s, len, &mut res);
            }
        }

        res
    }
}

// impl Runable for Solution {
//     fn run() {
//         assert_eq!(
//             Self::longest_palindrome(String::from("babad")),
//             String::from("bab")
//         );
//
//         assert_eq!(
//             Self::longest_palindrome(String::from("cbbd")),
//             String::from("bb")
//         );
//     }
// }
