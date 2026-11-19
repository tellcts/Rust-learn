//! 移动零

pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut [i32]) {
        let mut left = 0;

        for right in 0..nums.len() {
            if nums[right] != 0 {
                nums.swap(left, right);
                left += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&mut [0], &[0])]
    #[case(&mut [0, 0, 1], &[1, 0, 0])]
    #[case(&mut [0, 1, 0, 3, 12], &[1, 3, 12, 0, 0])]
    fn test_move_zeroes(#[case] nums: &mut [i32], #[case] expect: &[i32]) {
        Solution::move_zeroes(nums);
        assert_eq!(nums, expect);
    }
}
