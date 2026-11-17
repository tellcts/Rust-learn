//! 滑动窗口最大值

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut deq = VecDeque::new();
        let mut ans = Vec::with_capacity(nums.len() - k + 1);

        for (idx, &num) in nums.iter().enumerate() {
            while !deq.is_empty() && nums[*deq.back().unwrap()] <= num {
                deq.pop_back();
            }
            deq.push_back(idx);

            if deq[0] + k <= idx {
                deq.pop_front();
            }

            if idx >= k - 1 {
                ans.push(nums[deq[0]]);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1], 1, vec![1])]
    #[case(vec![1, 3, -1, -3, 5, 3, 6, 7], 3, vec![3, 3, 5, 5, 6, 7])]
    fn test_max_sliding_window(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: Vec<i32>) {
        assert_eq!(Solution::max_sliding_window(nums, k), expected);
    }
}
