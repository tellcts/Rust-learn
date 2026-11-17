//! 轮转数组

pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut [i32], k: i32) {
        let k = k as usize % nums.len();
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&mut [1,2], 3, &[2,1])]
    #[case(&mut [1,2,3,4,5,6,7], 3, &[5,6,7,1,2,3,4])]
    fn test_rotate(#[case] nums: &mut [i32], #[case] k: i32, #[case] expected: &[i32]) {
        Solution::rotate(nums, k);
        assert_eq!(nums, expected);
    }
}
