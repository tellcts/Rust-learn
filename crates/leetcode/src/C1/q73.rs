//! 矩阵置零

pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
        let (row, col) = (matrix.len(), matrix[0].len());
        let first_row_has_zero = matrix[0].contains(&0);

        for i in 1..row {
            for j in 0..col {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in 1..row {
            for j in (0..col).rev() {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if first_row_has_zero {
            matrix[0].fill(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        &mut [vec![1, 1, 1]
             ,vec![1, 0, 1]
             ,vec![1, 1, 1]]
             ,
            &[vec![1, 0, 1]
             ,vec![0, 0, 0]
             ,vec![1, 0, 1]] 
    )]
    #[case(
        &mut [vec![0, 1, 2, 0]
             ,vec![3, 4, 5, 2]
             ,vec![1, 3, 1, 5]]
             ,
            &[vec![0, 0, 0, 0]
             ,vec![0, 4, 5, 0]
             ,vec![0, 3, 1, 0]]
    )]
    fn test_set_zeroes(#[case] matrix: &mut [Vec<i32>], #[case] expected: &[Vec<i32>]){
        Solution::set_zeroes(matrix);
        assert_eq!(matrix,expected);
    }

}


