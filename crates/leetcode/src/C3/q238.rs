//! 除自身以外的数组乘积

pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut suffix = vec![1; len];

        for idx in (0..len - 1).rev() {
            suffix[idx] = suffix[idx + 1] * nums[idx + 1];
        }

        let mut prefix = 1;

        for (idx, num) in nums.iter().enumerate() {
            suffix[idx] *= prefix;
            prefix *= num;
        }

        suffix
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1, 2], vec![2, 1])]
    #[case(vec![1, 2, 3, 4], vec![24, 12, 8, 6])]
    #[case(vec![6, 1, 2, 3, 0, 5], vec![0, 0, 0, 0, 180, 0])]
    fn test_product_except_self(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        assert_eq!(Solution::product_except_self(nums), expected);
    }
}
