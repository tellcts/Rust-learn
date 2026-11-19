//! 两数相加

use crate::ListNode;

pub struct Solution;

impl Solution {
    /// 递归解法
    pub fn add_two_numbers_recursive(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two(l1, l2, 0)
    }

    fn add_two(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && carry == 0 {
            return None;
        }

        let mut sum = carry;

        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }

        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }

        Some(Box::new(ListNode {
            val: sum % 10,
            next: Self::add_two(l1, l2, sum / 10),
        }))
    }

    /// 迭代解法
    pub fn add_two_numbers_iterative(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut guard = ListNode::new(0);
        let mut cur = &mut guard;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }

            cur.next = Some(Box::new(ListNode::new(sum % 10)));
            carry = sum / 10;
            cur = cur.next.as_mut()?;
        }

        guard.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list_from_vec;
    use rstest::rstest;
    use rstest_reuse::{apply, template};

    #[template]
    #[rstest]
    #[case(vec![2,4,3],vec![5,6,4],vec![7,0,8])]
    #[case(vec![0],vec![0],vec![0])]
    #[case(vec![9,9,9,9,9,9,9],vec![9,9,9,9],vec![8, 9, 9, 9, 0, 0, 0, 1])]
    fn test_cases(#[case] l1: Vec<i32>, #[case] l2: Vec<i32>, #[case] expect: Vec<i32>) {}

    #[apply(test_cases)]
    fn test_add_two_numbers_iterative(l1: Vec<i32>, l2: Vec<i32>, expect: Vec<i32>) {
        assert_eq!(
            Solution::add_two_numbers_iterative(list_from_vec(l1), list_from_vec(l2)),
            list_from_vec(expect)
        );
    }

    #[apply(test_cases)]
    fn test_add_two_numbers_recursive(l1: Vec<i32>, l2: Vec<i32>, expect: Vec<i32>) {
        assert_eq!(
            Solution::add_two_numbers_recursive(list_from_vec(l1), list_from_vec(l2)),
            list_from_vec(expect)
        );
    }
}
