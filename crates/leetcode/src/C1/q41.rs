//! 缺失的第一个正数

pub struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();

        for i in 0..len {
            while nums[i] > 0 && nums[i] <= len as i32 && nums[(nums[i] - 1) as usize] != nums[i] {
                let j = (nums[i] - 1) as usize;
                nums.swap(i, j);
            }
        }

        for (i, &num) in nums.iter().enumerate() {
            if num != i as i32 + 1 {
                return i as i32 + 1;
            }
        }

        (len + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1, 2, 0], 3)]
    #[case(vec![3, 4, -1, 1], 2)]
    #[case(vec![7, 8, 9, 11, 12], 1)]
    fn test_first_missing_positive(#[case] nums: Vec<i32>, #[case] expected: i32) {
        assert_eq!(Solution::first_missing_positive(nums), expected);
    }
}
