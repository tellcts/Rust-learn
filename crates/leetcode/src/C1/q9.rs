//! 回文数

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut tmp = x;
        let mut rev = 0;

        while tmp > 0 {
            rev = rev * 10 + tmp % 10;
            tmp /= 10;
        }

        rev == x
    }

    pub fn is_palindrome_optimize(mut x: i32) -> bool {
        if x < 0 || (x > 0 && x % 10 == 0) {
            return false;
        }

        let mut rev = 0;
        while rev < x / 10 {
            rev = rev * 10 + x % 10;
            x /= 10;
        }

        rev == x || rev == x / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rstest_reuse::{apply, template};

    #[template]
    #[rstest]
    #[case(121, true)]
    #[case(-121,false)]
    #[case(10, false)]
    #[case(12321, true)]
    fn test_cases(#[case] x: i32, #[case] expect: bool) {}

    #[apply(test_cases)]
    fn test_is_palindrome(x: i32, expect: bool) {
        assert_eq!(Solution::is_palindrome(x), expect);
    }

    #[apply(test_cases)]
    fn test_is_palindrome_optimize(x: i32, expect: bool) {
        assert_eq!(Solution::is_palindrome_optimize(x), expect);
    }
}
