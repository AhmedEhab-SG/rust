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

// impl Runable for Solution {
//     fn run() {
//         let mut result_1 = Self::group_anagrams(vec![
//             "eat".to_string(),
//             "tea".to_string(),
//             "tan".to_string(),
//             "ate".to_string(),
//             "nat".to_string(),
//             "bat".to_string(),
//         ]);
//
//         let mut expected_1 = vec![
//             vec!["tan".to_string(), "nat".to_string()],
//             vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
//             vec!["bat".to_string()],
//         ];
//
//         result_1.iter_mut().for_each(|vec_s| vec_s.sort());
//         result_1.sort();
//
//         expected_1.iter_mut().for_each(|vec_s| vec_s.sort());
//         expected_1.sort();
//
//         assert_eq!(result_1, expected_1);
//
//         assert_eq!(
//             Self::group_anagrams(vec!["".to_string()]),
//             vec![vec!["".to_string()]]
//         );
//
//         assert_eq!(
//             Self::group_anagrams(vec!["a".to_string()]),
//             vec![vec!["a".to_string()]]
//         );
//     }
// }
