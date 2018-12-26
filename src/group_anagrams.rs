use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .fold(HashMap::new(), |mut map, s| {
                map.entry(s.bytes().fold([0; 26], |mut hash, b| {
                    hash[(b - b'a') as usize] += 1u8;
                    hash
                }))
                .or_insert_with(Vec::new)
                .push(s);
                map
            })
            .into_iter()
            .map(|s| s.1)
            .collect::<Vec<_>>()
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let mut ret = Solution::group_anagrams(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        ret.iter_mut().for_each(|v|v.sort_unstable());
        ret.sort_unstable();
        let ans = vec![vec!["ate", "eat", "tea"], vec!["bat"], vec!["nat", "tan"]];
        assert_eq!(ret, ans);
    }
}
