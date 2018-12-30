//! `[1, 2, 3]` 的全排列为
//! 1 + `[2, 3]` 的全排列 +
//! 2 + `[1, 3]` 的全排列 +
//! 3 + `[2, 3]` 的全排列
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            vec![vec![nums[0]]]
        } else {
            nums.iter().map(|i| {
                let arg = nums.iter().filter_map(|n| if n != i { Some(*n) } else { None }).collect();;
                let mut sub = Self::permute(arg);
                sub.iter_mut().for_each(|v| v.push(*i));
                sub
            }).flatten().collect()
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let mut ret = Solution::permute(vec![1, 2, 3]);
        ret.sort_unstable();

        let mut ans = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ];
        ans.sort_unstable();

        assert_eq!(ret, ans);
    }
}