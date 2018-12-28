#[derive(Debug)]
pub struct Trie {
    next: ([Option<Box<Trie>>; 26], bool)
}

/** You can modify the type of `self` for your need. */
impl Trie {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self { next: Default::default() }
    }

    /** Inserts a word into the trie. */
    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.next;
        for idx in word.bytes().map(|b| (b - b'a') as usize) {
            let nodes = &mut node.0;
            let next = unsafe { nodes.get_unchecked_mut(idx) };
            if next.is_none() {
                *next = Some(Box::new(Trie::new()));
            }
            node = &mut next.as_mut().unwrap().next;
        }
        node.1 = true;
    }

    #[inline]
    fn find(&self, word: String) -> (bool, bool) {
        let mut node = &self.next;
        for idx in word.bytes().map(|b| (b - b'a') as usize) {
            let next = unsafe { node.0.get_unchecked(idx) };
            if let Some(next_node) = next.as_ref() {
                node = &next_node.next;
            } else {
                return (false, false);
            }
        }
        let x = node.1;
        (x, true)
    }

    /** Returns if the word is in the trie. */
    pub fn search(&self, word: String) -> bool {
        self.find(word).0
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    pub fn starts_with(&self, prefix: String) -> bool {
        self.find(prefix).1
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test() {
        let mut obj = Trie::new();
        obj.insert("apple".to_owned());
        assert_eq!(obj.search("apple".to_owned()), true);
        assert_eq!(obj.search("app".to_owned()), false);
        assert_eq!(obj.search("appled".to_owned()), false);
        assert_eq!(obj.starts_with("app".to_owned()), true);
        obj.insert("app".to_owned());
        assert_eq!(obj.search("app".to_owned()), true);
    }
}

#[cfg(all(feature = "bench", test))]
mod bench {
    extern crate test;
    use super::Trie;
    use self::test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| {
            let mut obj = Trie::new();
            obj.insert("appleappleappleapple".to_owned());
            obj.search("appleappleappleapple".to_owned());
            obj.search("appleapple".to_owned());
            obj.starts_with("appleapple".to_owned());
            obj.insert("appleapple".to_owned());
            obj.search("appleapple".to_owned());
        });
    }
}
