//! 反转链表

use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut cur = head;

        while let Some(mut node) = cur {
            let nxt = node.next;
            node.next = pre;
            pre = Some(node);
            cur = nxt;
        }

        pre
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list_from_vec;
    use rstest::rstest;

    #[rstest]
    #[case(list_from_vec(vec![]), list_from_vec(vec![]))]
    #[case(list_from_vec(vec![0]), list_from_vec(vec![0]))]
    #[case(list_from_vec(vec![1, 2, 3, 4, 5]), list_from_vec(vec![5, 4, 3, 2, 1]))]
    fn test_reverse_list(
        #[case] head: Option<Box<ListNode>>,
        #[case] expect: Option<Box<ListNode>>,
    ) {
        assert_eq!(Solution::reverse_list(head), expect);
    }
}
