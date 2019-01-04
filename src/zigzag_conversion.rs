impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let s = s.as_bytes();
        let mut ret = String::with_capacity(s.len());
        for n in 0..num_rows {
            let mut idx = 0;
            for &i in [n].iter().chain([(num_rows - n - 1) * 2, 2 * n].iter().cycle()) {
                if idx + i != idx || idx == 0 {
                    idx += i;
                    if idx as usize >= s.len() {
                        break;
                    }
                    ret.push(s[idx as usize] as char);
                }
            }
        }
        ret
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::convert("LEETCODEISHIRING".to_owned(), 3), "LCIRETOESIIGEDHN");
        assert_eq!(Solution::convert("LEETCODEISHIRING".to_owned(), 4), "LDREOEIIECIHNTSG");
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| Solution::convert("LEETCODEISHIRING".to_owned(), 3));
    }
}