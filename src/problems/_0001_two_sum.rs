use std::collections::HashMap;

#[allow(unused_imports)]
use crate::Runable;

use super::Solution;

// problem: https://leetcode.com/problems/two-sum/
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map: HashMap<i32, i32> = HashMap::new();

        // this will have map like {2: 0, 7: 1, 11: 2, 15: 3}
        for (i, &num) in nums.iter().enumerate() {
            // we need to find a number such that num + computed_value = target
            let computed_value = target - num;

            // if we have already seen the computed_value in the map, we can return the indices
            if let Some(&index) = num_map.get(&computed_value) {
                return vec![index as i32, i as i32];
            }

            num_map.insert(num, i as i32);
        }

        vec![]
    }
}

// impl Runable for Solution {
//     fn run() {
//         assert_eq!(Self::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
//     }
// }
