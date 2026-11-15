//! 整数反转

pub struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut ans: i32 = 0;

        while x != 0 {
            let tmp = match ans.checked_mul(10) {
                Some(val) => val,
                None => return 0,
            };

            ans = match tmp.checked_add(x % 10) {
                Some(val) => val,
                None => return 0,
            };

            x /= 10;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    #[case(120, 21)]
    #[case(123, 321)]
    #[case(-123,-321)]
    fn test_reverse(#[case] x: i32, #[case] expected: i32) {
        assert_eq!(Solution::reverse(x), expected);
    }
}
