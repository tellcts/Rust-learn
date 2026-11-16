//! 下一个排列

/// [灵茶山艾府](https://leetcode.cn/problems/next-permutation/solutions/3621022/jiao-ni-cong-ling-kai-shi-si-kao-zhe-ti-9qfrq/)
pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut [i32]) {
        let n = nums.len();
        let mut idx = n as isize - 2;

        while idx >= 0 && nums[idx as usize] >= nums[(idx + 1) as usize] {
            idx -= 1;
        }

        if idx >= 0 {
            let i = idx as usize;
            let mut j = n - 1;

            while nums[j] <= nums[i] {
                j -= 1;
            }

            nums.swap(i, j);
        }

        nums[(idx + 1) as usize..].reverse();
    }

    pub fn pre_permutation(nums: &mut [i32]) {
        let n = nums.len();
        let mut idx = n as isize - 2;

        while idx >= 0 && nums[idx as usize] <= nums[(idx + 1) as usize] {
            idx -= 1;
        }

        if idx >= 0 {
            let i = idx as usize;
            let mut j = n - 1;

            while nums[j] >= nums[i] {
                j -= 1;
            }

            nums.swap(i, j);
        }

        nums[(idx + 1) as usize..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&mut [1, 2, 3], &mut [1, 3, 2])]
    #[case(&mut [3, 2, 1], &mut [1, 2, 3])]
    #[case(&mut [1, 1, 5], &mut [1, 5, 1])]
    fn test_next_permutation(#[case] nums: &mut [i32], #[case] expected: &mut [i32]) {
        Solution::next_permutation(nums);
        assert_eq!(nums, expected);
    }

    #[rstest]
    #[case(&mut [1, 2, 3], &mut [3, 2, 1])]
    #[case(&mut [3, 2, 1], &mut [3, 1, 2])]
    #[case(&mut [1, 5, 1], &mut [1, 1, 5])]
    fn test_pre_permutation(#[case] nums: &mut [i32], #[case] expected: &mut [i32]) {
        Solution::pre_permutation(nums);
        assert_eq!(nums, expected);
    }
}
