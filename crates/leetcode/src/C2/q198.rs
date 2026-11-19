//! 打家劫舍

pub struct Solution;

impl Solution {
    pub fn rob_dp(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n + 2];

        for i in 0..n {
            dp[i + 2] = dp[i + 1].max(dp[i] + nums[i]);
        }

        dp[n + 1]
    }

    pub fn rob_iterative(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, 0);

        for num in nums {
            (a, b) = (b, b.max(a + num));
        }

        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rstest_reuse::{apply, template};

    #[template]
    #[rstest]
    #[case(vec![1,2,3,1], 4)]
    #[case(vec![2,7,9,3,1], 12)]
    fn test_cases(#[case] nums: Vec<i32>, #[case] expect: i32) {}

    #[apply(test_cases)]
    fn test_rob_dp(nums: Vec<i32>, expect: i32) {
        assert_eq!(Solution::rob_dp(nums), expect);
    }

    #[apply(test_cases)]
    fn test_rob_iterative(nums: Vec<i32>, expect: i32) {
        assert_eq!(Solution::rob_iterative(nums), expect);
    }
}
