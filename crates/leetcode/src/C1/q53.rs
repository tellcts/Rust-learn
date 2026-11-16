//! 最大子数组和

pub struct Solution;

impl Solution {
    /// Kadane's Algorithm
    pub fn max_sub_array_greedy(nums: Vec<i32>) -> i32 {
        let (mut ans, mut cur_sum) = (nums[0], nums[0]);

        for &item in nums.iter().skip(1) {
            cur_sum = item.max(cur_sum + item);
            ans = ans.max(cur_sum)
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1, 2, -1], 3)]
    #[case(vec![5, 4, -1, 7, 8], 23)]
    #[case(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], 6)]
    fn test_max_sub_array_greedy(#[case] nums: Vec<i32>, #[case] expected: i32) {
        assert_eq!(Solution::max_sub_array_greedy(nums), expected);
    }
}
