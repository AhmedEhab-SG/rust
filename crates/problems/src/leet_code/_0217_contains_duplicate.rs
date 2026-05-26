use std::collections::HashSet;

#[allow(unused_imports)]
use utils::Runable;

use crate::Solution;

// problem: https://leetcode.com/problems/contains-duplicate/description/
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::with_capacity(nums.len());

        for n in nums {
            if !set.insert(n) {
                return true;
            }
        }

        false
    }
}

// impl Runable for Solution {
//     fn run() {
//         assert_eq!(Self::contains_duplicate(vec![1, 2, 3, 1]), true);
//
//         assert_eq!(Self::contains_duplicate(vec![1, 2, 3, 4]), false);
//
//         assert_eq!(
//             Self::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
//             true
//         );
//     }
// }
