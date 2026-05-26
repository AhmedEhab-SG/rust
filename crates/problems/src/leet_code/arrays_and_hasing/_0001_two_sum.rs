use std::collections::HashMap;

#[allow(unused_imports)]
use utils::Runable;

use crate::Solution;

// problem: https://leetcode.com/problems/two-sum/
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_map = HashMap::<i32, i32>::with_capacity(nums.len());

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = nums_map.get(&(target - num)) {
                return vec![j as i32, i as i32];
            }

            nums_map.insert(num, i as i32);
        }

        unreachable!()
    }
}

// impl Runable for Solution {
//     fn run() {
//         assert_eq!(Self::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
//
//         assert_eq!(Self::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
//
//         assert_eq!(Self::two_sum(vec![3, 3], 6), vec![0, 1]);
//     }
// }
