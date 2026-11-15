//! 最长回文子串

pub struct Solution;

impl Solution {
    /// 中心扩展法
    /// 时间复杂度：O(n^2)
    /// 空间复杂度：O(1)
    pub fn longest_palindrome_1(s: &str) -> &str {
        if s.is_empty() {
            return "";
        }

        let n = s.len();
        let bytes = s.as_bytes();
        let (mut ans_left, mut ans_right) = (0, 0);

        for i in 0..(2 * n - 1) {
            let (mut left, mut right) = ((i / 2) as i32, i.div_ceil(2));

            while left >= 0 && right < n && bytes[left as usize] == bytes[right] {
                left -= 1;
                right += 1;
            }

            if (right as i32 - left - 1) as usize > ans_right - ans_left {
                ans_left = (left + 1) as usize;
                ans_right = right;
            }
        }

        &s[ans_left..ans_right]
    }

    // TODO: Manacher算法实现
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rstest_reuse::{apply, template};

    #[template]
    #[rstest]
    #[case("cbbd", "bb")]
    #[case("babad", "bab")]
    #[case("qwetzcanaczt", "tzcanaczt")]
    fn test_cases(#[case] s: &str, #[case] expected: &str) {}

    #[apply(test_cases)]
    fn test_longest_palindrome_1(s: &str, expected: &str) {
        assert_eq!(Solution::longest_palindrome_1(s), expected);
    }
}
