#[allow(unused_imports)]
use crate::Runable;

use super::Solution;

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged_array: Vec<_> = nums1.iter().chain(nums2.iter()).collect();

        let len = merged_array.len();
        let mid = len / 2;

        if len % 2 == 0 {
            merged_array.select_nth_unstable(mid - 1);
            let first = merged_array[mid - 1];

            merged_array.select_nth_unstable(mid);
            let second = merged_array[mid];

            Some((first + second) as f64 / 2.0).unwrap()
        } else {
            merged_array.select_nth_unstable(mid);
            Some(*merged_array[mid] as f64).unwrap()
        }
    }
}

impl Runable for Solution {
    fn run() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2 as f64
        );

        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5,
        );
    }
}
