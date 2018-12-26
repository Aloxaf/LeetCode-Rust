impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtrack(ret: &mut Vec<String>, now: String, left: i32, right: i32, n: i32) {
            if now.len() == 2 * n as usize {
                ret.push(now);
            } else {
                if left < n {
                    backtrack(ret, now.clone() + "(", left + 1, right, n);
                }
                if right < left {
                    backtrack(ret, now + ")", left, right + 1, n);
                }
            }
        }
        let mut ret = vec![];
        backtrack(&mut ret, "".to_owned(), 0, 0, n);
        ret
    }
}

impl Solution {
    // 和这个大佬说的dp很像
    // https://leetcode.com/problems/generate-parentheses/discuss/209410/c%2B%2B-dynamic-programming-(0ms)
    // 设 a[n] = generate_parenthesis(n)
    // a[0] = ""
    // a[n] = "(" + a[x] + ")" + a[y], x + y = n - 1
    pub fn generate_parenthesis_cn(n: i32) -> Vec<String> {
        if n == 0 {
            return vec!["".to_owned()];
        }
        let mut ret = vec![];
        for c in 0..n {
            for left in Solution::generate_parenthesis_cn(c) {
                for right in Solution::generate_parenthesis_cn(n - c - 1) {
                    ret.push(format!("({}){}", left, right));
                }
            }
        }
        ret
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::vec_string;

    #[test]
    fn test() {
        let mut ret = Solution::generate_parenthesis(3);
        ret.sort_unstable();
        assert_eq!(
            ret,
            vec_string!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn test_cn() {
        let mut ret = Solution::generate_parenthesis_cn(3);
        ret.sort_unstable();
        assert_eq!(
            ret,
            vec_string!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}

#[cfg(all(feature = "bench", test))]
mod bench {
    extern crate test;
    use super::Solution;
    use self::test::Bencher;

    #[bench]
    fn backtrack(b: &mut Bencher) {
        b.iter(|| Solution::generate_parenthesis(4));
    }

    #[bench]
    fn clourse_number(b: &mut Bencher) {
        b.iter(|| Solution::generate_parenthesis_cn(4));
    }
}
