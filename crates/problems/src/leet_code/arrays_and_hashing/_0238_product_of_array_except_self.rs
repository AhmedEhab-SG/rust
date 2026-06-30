#[allow(unused_imports)]
use utils::Runable;

use crate::Solution;

// problem: https://leetcode.com/problems/product-of-array-except-self
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // let mut answer = Vec::with_capacity(nums.len());
        // let mut temp_nums = nums.clone();
        //
        // for (i, _) in nums.iter().enumerate() {
        //     let temp_value = temp_nums[i];
        //     temp_nums.remove(i);
        //
        //     let mut sum = 1;
        //
        //     for temp_num in temp_nums.iter() {
        //         sum *= temp_num;
        //     }
        //
        //     answer.push(sum);
        //     temp_nums.insert(i, temp_value);
        // }
        //
        // answer

        let n = nums.len();
        let mut answer = vec![1; n];

        // pass 1: fill answer with prefix products
        let mut prefix = 1;
        for i in 0..n {
            answer[i] = prefix;
            prefix *= nums[i];
        }
        //  after pass 1: answer = [1, 1, 2, 6]

        // pass 2: multiply each by suffix product from the right
        let mut suffix = 1;
        for i in (0..n).rev() {
            answer[i] *= suffix;
            suffix *= nums[i];
        }
        //  after pass 2: answer = [24, 12, 8, 6]

        answer
    }
}

// impl Runable for Solution {
//     fn run() {
//         assert_eq!(
//             Self::product_except_self(vec![1, 2, 3, 4]),
//             vec![24, 12, 8, 6]
//         );
//
//         assert_eq!(
//             Self::product_except_self(vec![-1, 1, 0, -3, 3]),
//             vec![0, 0, 9, 0, 0]
//         );
//     }
// }
