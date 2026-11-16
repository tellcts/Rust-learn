//! 找出字符串中第一个匹配项的下标

pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let (s_bytes, p_bytes) = (haystack.as_bytes(), needle.as_bytes());
        let (s_len, p_len) = (s_bytes.len(), p_bytes.len());

        for i in 0..((s_len + 1).saturating_sub(p_len)) {
            if &s_bytes[i..i + p_len] == p_bytes {
                return i as _;
            }
        }

        -1
    }

    #[rustfmt::skip]
    pub fn str_str_opt(haystack: String, needle: String) -> i32 {
        haystack
            .find(&needle)
            .map_or(-1, |pos| pos as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rstest_reuse::{apply, template};

    #[template]
    #[rstest]
    #[case(String::from("sadbutsad"), String::from("sad"), 0)]
    #[case(String::from("leetcode"), String::from("leeto"), -1)]
    #[case(String::from("tellcts"), String::from("tellctsespain"), -1)]
    fn test_cases(#[case] haystack: String, #[case] needle: String, #[case] expected: i32) {}

    #[apply(test_cases)]
    fn test_str_str(haystack: String, needle: String, expected: i32) {
        assert_eq!(Solution::str_str(haystack, needle), expected);
    }

    #[apply(test_cases)]
    fn test_str_str_opt(haystack: String, needle: String, expected: i32) {
        assert_eq!(Solution::str_str_opt(haystack, needle), expected);
    }
}
