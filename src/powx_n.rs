impl Solution {
    pub fn my_pow(mut x: f64, mut n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if n < 0 {
            n = if n != std::i32::MIN { -n } else { std::i32::MAX - 1 }; // 奇偶相同
            x = 1.0 / x;
        }
        if n % 2 == 0 {
            Self::my_pow(x * x, n / 2)
        } else {
            x * Self::my_pow(x * x, n / 2)
        }
    }

    pub fn my_pow_bit(mut x: f64, mut n: i32) -> f64 {
        if n < 0 {
            n = if n != std::i32::MIN { -n } else { std::i32::MAX - 1 }; // 奇偶相同
            x = 1.0 / x;
        }
        let mut ret = 1.0;
        while n != 0 {
            if n & 1 == 1 {
                ret *= x;
            }
            x *= x;
            n >>= 1;
        }
        ret
    }

    pub fn my_pow_normal(mut x: f64, mut n: i32) -> f64 {
        if n < 0 {
            n = if n != std::i32::MIN { -n } else { std::i32::MAX - 1 }; // 奇偶相同
            x = 1.0 / x;
        }
        (0..n).fold(1.0, |ret, _| ret * x)
    }

    pub fn my_pow_builtin(x: f64, n: i32) -> f64 {
        x.powi(n)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn builtin() {
        assert!(Solution::my_pow_builtin(2.0, 10) - 1024.0 < 0.00001);
        assert!(Solution::my_pow_builtin(2.1, 3) - 9.261 < 0.00001);
        assert!(Solution::my_pow_builtin(2.0, -2) - 0.25 < 0.00001);
    }

    #[test]
    fn normal() {
        assert!(Solution::my_pow_normal(2.0, 10) - 1024.0 < 0.00001);
        assert!(Solution::my_pow_normal(2.1, 3) - 9.261 < 0.00001);
        assert!(Solution::my_pow_normal(2.0, -2) - 0.25 < 0.00001);
    }

    #[test]
    fn bin() {
        assert!(Solution::my_pow(2.0, 10) - 1024.0 < 0.00001);
        assert!(Solution::my_pow(2.1, 3) - 9.261 < 0.00001);
        assert!(Solution::my_pow(2.0, -2) - 0.25 < 0.00001);
    }

    #[test]
    fn bit() {
        assert!(Solution::my_pow_bit(2.0, 10) - 1024.0 < 0.00001);
        assert!(Solution::my_pow_bit(2.1, 3) - 9.261 < 0.00001);
        assert!(Solution::my_pow_bit(2.0, -2) - 0.25 < 0.00001);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use super::Solution;
    use self::test::{Bencher, black_box};

    #[bench]
    fn builtin(b: &mut Bencher) {
        b.iter(|| Solution::my_pow_builtin(black_box(1.234), black_box(100)));
    }

    #[bench]
    fn normal(b: &mut Bencher) {
        b.iter(|| Solution::my_pow_normal(black_box(1.234), black_box(100)));
    }

    #[bench]
    fn bin(b: &mut Bencher) {
        b.iter(|| Solution::my_pow(black_box(1.234), black_box(100)));
    }

    #[bench]
    fn bit(b: &mut Bencher) {
        b.iter(|| Solution::my_pow_bit(black_box(1.234), black_box(100)));
    }
}
