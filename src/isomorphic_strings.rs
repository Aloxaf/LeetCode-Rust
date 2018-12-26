impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map1 = [0; 255];
        let mut map2 = [0; 255];
        for (a, b) in s.bytes().zip(t.bytes()) {
            if (map1[a as usize] != 0 || map2[b as usize] != 0)
                && (map1[a as usize] != b || map2[b as usize] != a)
            {
                return false;
            }
            map1[a as usize] = b;
            map2[b as usize] = a;
        }
        true
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_isomorphic("egg".to_owned(), "add".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_isomorphic("foo".to_owned(), "bar".to_owned()),
            false
        );
        assert_eq!(
            Solution::is_isomorphic("paper".to_owned(), "title".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_isomorphic("aa".to_owned(), "ab".to_owned()),
            false
        );
    }
}
