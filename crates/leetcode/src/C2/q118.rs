//! 杨辉三角

pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = num_rows as usize;
        let mut ans = vec![vec![]; n];

        for i in 0..n {
            ans[i].resize(i + 1, 1);

            for j in 1..i {
                ans[i][j] = ans[i - 1][j - 1] + ans[i - 1][j];
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(5,vec![vec![1],
                  vec![1, 1],
                  vec![1, 2, 1],
                  vec![1, 3, 3, 1],
                  vec![1, 4, 6, 4, 1]]
    )]
    fn test_generate(#[case] num_rows: i32, #[case] expect: Vec<Vec<i32>>) {
        assert_eq!(Solution::generate(num_rows), expect);
    }
}
