impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut x = (1, 1);
        for _ in 0..n {
            x = (x.1, x.0 + x.1);
        }
        x.0
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    
    #[test]
    fn test() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}