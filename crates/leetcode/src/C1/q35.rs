//! 搜索插入位置

/// [灵茶山艾府](https://leetcode.cn/problems/search-insert-position/solutions/2023391/er-fen-cha-zhao-zong-shi-xie-bu-dui-yi-g-nq23/)
pub struct Solution;

impl Solution {
    pub fn search_insert(nums: &[i32], target: i32) -> usize {
        Self::lower_bound(nums, target)
    }

    fn lower_bound(nums: &[i32], target: i32) -> usize {
        let (mut left, mut right) = (0, nums.len());

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&[1,3,5,6], 5, 2)]
    #[case(&[1,3,5,6], 2, 1)]
    #[case(&[1,3,5,6], 7, 4)]
    fn test_search_insert(#[case] nums: &[i32], #[case] target: i32, #[case] expect: usize) {
        assert_eq!(Solution::search_insert(nums, target), expect);
    }
}
