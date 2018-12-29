impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        // http://www.cnblogs.com/grandyang/p/5294255.html
        let mut ret = vec![0; num as usize + 1];
        for i in 1..=(num as usize) {
            ret[i] = ret[i & (i - 1)] + 1;
        }
        ret
    }

    pub fn count_bits_builtin(num: i32) -> Vec<i32> {
        (0..=num).map(|n|n.count_ones() as i32).collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use super::Solution;
    use self::test::Bencher;

    #[bench]
    fn custom(b: &mut Bencher) {
        b.iter(|| Solution::count_bits(500));
    }

    #[bench]
    fn builtin(b: &mut Bencher) {
        b.iter(|| Solution::count_bits_builtin(500));
    }
}
