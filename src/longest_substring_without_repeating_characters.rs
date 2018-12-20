use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();
        let (mut i, mut ans) = (0, 0);
        for (j, c) in s.chars().enumerate() {
            i = i.max(*map.get(&c).unwrap_or(&0));
            ans = ans.max(j - i + 1);
            map.insert(c, j + 1);
        }
        ans as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
    }
}