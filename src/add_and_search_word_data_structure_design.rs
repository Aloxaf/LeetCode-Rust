/// 使用上次写的前缀树来实现
pub struct WordDictionary {
    trie: Trie
}

#[derive(Default)]
pub struct Trie {
    next: ([Option<Box<Trie>>; 26], bool)
}

/** You can modify the type of `self` for your need. */
impl WordDictionary {

    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            trie: Default::default()
        }
    }

    /** Adds a word into the data structure. */
    pub fn add_word(&mut self, word: String) {
        let mut node = &mut self.trie.next;
        for idx in word.bytes().map(|b| (b - b'a') as usize) {
            let nodes = &mut node.0;
            let next = unsafe { nodes.get_unchecked_mut(idx) };
            if next.is_none() {
                *next = Some(Box::new(Default::default()));
            }
            node = &mut next.as_mut().unwrap().next;
        }
        node.1 = true;
    }

    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    pub fn search(&self, word: String) -> bool {
        Self::find(&self.trie.next, &word)
    }

    fn find(trie: &([Option<Box<Trie>>; 26], bool), word: &str) -> bool {
        let mut node = trie;
        for (i, b) in word.bytes().enumerate() {
            // 遇到 '.' 则对所有的子节点调用 find
            if b == b'.' {
                for next in node.0.iter() {
                    if let Some(next_node) = next {
                        if Self::find(&next_node.next, &word[i+1..]) {
                            return true;
                        }
                    }
                }
                return false;
            } else {
                let next = unsafe { node.0.get_unchecked((b - b'a') as usize) };
                if let Some(next_node) = next.as_ref() {
                    node = &next_node.next;
                } else {
                    return false;
                }
            }
        }
        node.1
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode_test;
    use super::WordDictionary;

    #[test]
    fn test() {
        leetcode_test!(
            ["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
            [[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
            [null, null, null, null, false, true, true, true]
        );
    }
}