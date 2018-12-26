impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        s.bytes().fold(0, |n, b| n * 26 + (b - b'A' + 1) as i32)
    }

    pub fn title_to_number2(s: String) -> i32 {
        s.bytes().enumerate().fold(0, |n, b| n + (b.1 - b'A' + 1) as i32 * 26i32.pow((s.len() - b.0 - 1) as u32))
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::title_to_number("A".to_owned()), 1);
        assert_eq!(Solution::title_to_number("AB".to_owned()), 28);
        assert_eq!(Solution::title_to_number("ZY".to_owned()), 701);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::title_to_number2("A".to_owned()), 1);
        assert_eq!(Solution::title_to_number2("AB".to_owned()), 28);
        assert_eq!(Solution::title_to_number2("ZY".to_owned()), 701);
    }
}

#[cfg(all(feature = "bench", test))]
mod bench {
    extern crate test;
    use super::Solution;
    use self::test::Bencher;

    #[bench]
    fn test1(b: &mut Bencher) {
        b.iter(|| Solution::title_to_number("ABCD".to_owned()));
    }

    #[bench]
    fn test2(b: &mut Bencher) {
        b.iter(|| Solution::title_to_number2("ABCD".to_owned()));
    }
}
