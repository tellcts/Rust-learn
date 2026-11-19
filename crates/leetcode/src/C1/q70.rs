//! 爬楼梯

pub struct Solution;

impl Solution {
    pub fn climb_stairs_dp(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];

        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[n]
    }

    /// Space Optimization: O(n) -> O(1)
    pub fn climb_stairs_optimize(n: i32) -> i32 {
        let (mut a, mut b) = (1, 1);

        for _ in 2..=n {
            (a, b) = (b, a + b);
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
    #[case(2, 2)]
    #[case(3, 3)]
    fn test_cases(#[case] n: i32, #[case] expect: i32) {}

    #[apply(test_cases)]
    fn test_climb_stairs_dp(n: i32, expect: i32) {
        assert_eq!(Solution::climb_stairs_dp(n), expect);
    }

    #[apply(test_cases)]
    fn test_climb_stairs_optimize(n: i32, expect: i32) {
        assert_eq!(Solution::climb_stairs_optimize(n), expect);
    }
}
