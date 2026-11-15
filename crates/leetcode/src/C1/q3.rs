//! 无重复字符的最长子串

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: &str) -> usize {
        let s = s.as_bytes();
        let mut cnt = [0; 128];
        let (mut ans, mut left) = (0, 0);

        for (right, &ch) in s.iter().enumerate() {
            let ch = ch as usize;
            cnt[ch] += 1;

            while cnt[ch] > 1 {
                cnt[s[left] as usize] -= 1;
                left += 1;
            }

            ans = ans.max(right - left + 1);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("bbbbb", 1)]
    #[case("pwwkew", 3)]
    #[case("abcabcbb", 3)]
    fn test_length_of_longest_substring(#[case] s: &str, #[case] expected: usize) {
        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }
}
