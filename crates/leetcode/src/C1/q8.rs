//! 字符串转换整数（atoi）

pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut idx = 0;

        // 跳过前导空格
        while idx < n && s[idx] == b' ' {
            idx += 1;
        }

        // 处理符号
        let mut sign = 1;
        if idx < n && (s[idx] == b'+' || s[idx] == b'-') {
            if s[idx] == b'-' {
                sign = -1;
            }

            idx += 1;
        }

        // 处理数字
        let mut ans = 0;
        while idx < n && s[idx].is_ascii_digit() {
            let digit = (s[idx] - b'0') as i32;

            if ans > i32::MAX / 10 || ans * 10 > i32::MAX - digit {
                return if sign > 0 { i32::MAX } else { i32::MIN };
            }

            ans = ans * 10 + digit;
            idx += 1;
        }

        sign * ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(String::from("42"), 42)]
    #[case(String::from("   -42"),-42)]
    #[case(String::from("-042"),-42)]
    #[case(String::from("4193 with words"), 4193)]
    #[case(String::from("words and 987"), 0)]
    #[case(String::from("1337c0d3"), 1337)]
    fn test_my_atoi(#[case] s: String, #[case] expect: i32) {
        assert_eq!(Solution::my_atoi(s), expect);
    }
}
