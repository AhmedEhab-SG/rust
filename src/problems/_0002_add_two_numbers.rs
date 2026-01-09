#[allow(unused_imports)]
use crate::Runable;

use super::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }

    fn from_vec(vec: Vec<i32>) -> Option<Box<Self>> {
        let mut curr = None;
        for &val in vec.iter().rev() {
            let mut node = Self::new(val);
            node.next = curr;
            curr = Some(Box::new(node));
        }
        curr
    }
}

type List = Option<Box<ListNode>>;

// problem: https://leetcode.com/problems/add-two-numbers/
impl Solution {
    pub fn add_two_numbers(l1: List, l2: List) -> List {
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut carry = 0; // will carry the value if its more than 10
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = &node.next;
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = &node.next
            }
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
        }

        dummy.next
    }
}

impl Runable for Solution {
    fn run() {
        assert_eq!(
            Solution::add_two_numbers(
                ListNode::from_vec(vec![2, 4, 3]),
                ListNode::from_vec(vec!(5, 6, 4))
            ),
            ListNode::from_vec(vec![7, 0, 8])
        )
    }
}
