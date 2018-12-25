impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut flag = vec![true; n as usize];
        let max = (n as f32).sqrt().round() as usize;
        for i in 2..=max {
            if flag[i] {
                for j in (2 * i..n as usize).step_by(i) {
                    flag[j] = false;
                }
            }
        }
        flag.iter().filter(|&&b| b).count().saturating_sub(2) as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::count_primes(10), 4);
        assert_eq!(Solution::count_primes(1), 0);
    }
}
