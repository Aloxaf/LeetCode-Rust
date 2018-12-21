impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // 仿佛在刷 CodeWars (
        nums.iter().enumerate().filter(|(_, n)| **n >= target).next().unwrap_or((nums.len(), &0)).0 as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}