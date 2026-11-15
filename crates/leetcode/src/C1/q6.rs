//! Z字形变换

pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: usize) -> String {
        let mut rows = vec![String::new(); num_rows];
        let iter = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();

        iter.zip(s.chars()).for_each(|(i, ch)| rows[i].push(ch));

        rows.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(String::from("A"), 1, String::from("A"))]
    #[case(String::from("PAYPALISHIRING"), 3, String::from("PAHNAPLSIIGYIR"))]
    #[case(String::from("PAYPALISHIRING"), 4, String::from("PINALSIGYAHRPI"))]
    fn test_convert(#[case] s: String, #[case] num_rows: usize, #[case] expected: String) {
        assert_eq!(Solution::convert(s, num_rows), expected);
    }
}
