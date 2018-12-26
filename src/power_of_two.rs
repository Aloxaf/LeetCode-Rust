impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        return if n <= 0 {
            false
        } else {
            n & (n - 1) == 0
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::is_power_of_two(1), true);
        assert_eq!(Solution::is_power_of_two(8), true);
        assert_eq!(Solution::is_power_of_two(218), false);
    }
}
