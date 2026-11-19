//! 搜索二维矩阵

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (row, col) = (matrix.len(), matrix[0].len());
        let (mut x, mut y) = (0, col - 1);

        while x < row && y < col {
            match matrix[x][y].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => x += 1,
                _ => {
                    if y != 0 {
                        y -= 1;
                    } else {
                        return false;
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]], 3, true)]
    #[case(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]], 13, false)]
    fn test_search_matrix(
        #[case] matrix: Vec<Vec<i32>>,
        #[case] target: i32,
        #[case] expect: bool,
    ) {
        assert_eq!(Solution::search_matrix(matrix, target), expect);
    }
}
