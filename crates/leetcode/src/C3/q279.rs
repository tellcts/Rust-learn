//! 完全平方数

const N: usize = 10000;

const fn init_dp() -> [i32; N + 1] {
    let mut dp = [i32::MAX >> 1; N + 1];
    dp[0] = 0;

    let mut i = 1;

    while i <= 100 {
        let mut j = i * i;

        while j <= N {
            let candidate = dp[j - i * i] + 1;

            dp[j] = if dp[j] < candidate { dp[j] } else { candidate };
            j += 1;
        }

        i += 1;
    }

    dp
}

static DP: [i32; N + 1] = init_dp();

pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        DP[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(12, 3)]
    #[case(13, 2)]
    fn test_num_squares(#[case] n: i32, #[case] expect: i32) {
        assert_eq!(Solution::num_squares(n), expect);
    }
}
