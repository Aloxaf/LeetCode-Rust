impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = f64::from(x);
        let (mut a1, mut a2) = (1.0f64, 0.0f64);

        while (a1 - a2).abs() >= 0.1 {
            a2 = a1;
            a1 = (a1 + x / a1) / 2.0;
        }
        a1 as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}