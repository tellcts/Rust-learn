//! 整数转罗马数字

/// [灵茶山艾府](https://leetcode.cn/problems/integer-to-roman/solutions/2848775/jian-ji-xie-fa-pythonjavaccgojsrust-by-e-kmp6/)
pub struct Solution;

impl Solution {
    const ONES: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    const TENS: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    const HUNDREDS: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"];

    pub fn int_to_roman(num: i32) -> String {
        let n = num as usize;

        format!(
            "{}{}{}{}",
            Self::THOUSANDS[n / 1000],
            Self::HUNDREDS[n / 100 % 10],
            Self::TENS[n / 10 % 10],
            Self::ONES[n % 10]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(3, String::from("III"))]
    #[case(4, String::from("IV"))]
    #[case(9, String::from("IX"))]
    #[case(58, String::from("LVIII"))]
    #[case(1994, String::from("MCMXCIV"))]
    fn test_int_to_roman(#[case] num: i32, #[case] expected: String) {
        assert_eq!(Solution::int_to_roman(num), expected);
    }
}
