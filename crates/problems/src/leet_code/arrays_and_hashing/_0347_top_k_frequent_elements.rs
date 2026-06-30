use std::collections::HashMap;

#[allow(unused_imports)]
use utils::Runable;

use crate::Solution;

// problem: https://leetcode.com/problems/top-k-frequent-elements/
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for num in nums {
            nums_map.entry(num).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut result = nums_map.into_iter().collect::<Vec<_>>();

        result.sort_by(|a, b| b.1.cmp(&a.1));

        result
            .iter()
            .take(k as usize)
            .map(|(key, _)| *key)
            .collect()
    }
}

// impl Runable for Solution {
//     fn run() {
//         assert_eq!(Self::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
//
//         assert_eq!(Self::top_k_frequent(vec![1], 1), vec![1]);
//
//         assert_eq!(
//             Self::top_k_frequent(vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 2], 2),
//             vec![1, 2]
//         );
//     }
// }
