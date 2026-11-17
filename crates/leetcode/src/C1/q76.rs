//! 最小覆盖子串

pub struct Solution;

impl Solution {
    pub fn min_window<'a>(s: &'a str, t: &str) -> &'a str {
        let mut kinds = 0;
        let mut diff = [0; 128];

        for ch in t.bytes() {
            let ch = ch as usize;
            if diff[ch] == 0 {
                kinds += 1;
            }
            diff[ch] -= 1;
        }

        let s = s.as_bytes();
        let s_len = s.len();
        let (mut left, mut ge_cnt) = (0, 0);
        let (mut ans_left, mut ans_right) = (0, s_len);

        for (right, &ch) in s.iter().enumerate() {
            let ch = ch as usize;
            diff[ch] += 1;
            if diff[ch] == 0 {
                ge_cnt += 1;
            }

            while ge_cnt == kinds {
                if right - left < ans_right - ans_left {
                    ans_left = left;
                    ans_right = right;
                }

                let x = s[left] as usize;
                if diff[x] == 0 {
                    ge_cnt -= 1;
                }
                diff[x] -= 1;
                left += 1;
            }
        }

        if ans_right < s_len {
            unsafe { std::str::from_utf8_unchecked(&s[ans_left..=ans_right]) }
        } else {
            ""
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("a", "a", "a")]
    #[case("a", "aa", "")]
    #[case("ADOBECODEBANC", "ABC", "BANC")]
    fn test_min_window(#[case] s: &str, #[case] t: &str, #[case] expected: &str) {
        assert_eq!(Solution::min_window(s, t), expected);
    }
}
