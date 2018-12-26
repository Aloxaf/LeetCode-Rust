impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.bytes().rev().skip_while(|&c| c == b' ').take_while(|&c| c != b' ' ).count() as i32
    }

    pub fn length_of_last_word_builtin(s: String) -> i32 {
        s.trim().split_whitespace().last().map(|s| s.len()).unwrap_or(0) as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_owned()), 5);
        assert_eq!(Solution::length_of_last_word("a ".to_owned()), 1);
    }
}

#[cfg(all(feature = "bench", test))]
mod bench {
    extern crate test;
    use super::Solution;
    use self::test::Bencher;

    #[bench]
    fn custom(b: &mut Bencher) {
        b.iter(|| Solution::length_of_last_word("Hello World Hello World Hello World Hello World Hello World Hello World Hello World".to_owned()));
    }

    #[bench]
    fn builtin(b: &mut Bencher) {
        b.iter(|| Solution::length_of_last_word_builtin("Hello World Hello World Hello World Hello World Hello World Hello World Hello World".to_owned()));
    }
}
