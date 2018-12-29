impl Solution {
    pub fn reverse_string(s: String) -> String {
        s.chars().rev().collect()
    }

    // 这都不是最快? 第一的大佬究竟用了什么黑魔法...
    pub fn reverse_string_unsafe(mut s: String) -> String {
        unsafe {
            let s = s.as_bytes_mut();
            s.reverse();
            String::from_utf8_unchecked(s.to_vec())
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::reverse_string("hello".to_owned()), "olleh");
        assert_eq!(Solution::reverse_string("helo".to_owned()), "oleh");
    }

    #[test]
    fn test_unsafe() {
        assert_eq!(Solution::reverse_string_unsafe("hello".to_owned()), "olleh");
        assert_eq!(Solution::reverse_string_unsafe("helo".to_owned()), "oleh");
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| Solution::reverse_string("Hello World Hello World Hello World Hello World Hello World Hello World Hello World".to_owned()));
    }

    #[bench]
    fn bench_unsafe(b: &mut Bencher) {
        b.iter(|| Solution::reverse_string_unsafe("Hello World Hello World Hello World Hello World Hello World Hello World Hello World".to_owned()));
    }
}
