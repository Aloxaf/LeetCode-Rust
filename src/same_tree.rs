use crate::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    // Best Practice !
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => p == q,
            (None, None) => true,
            _ => false,
        }
    }

    pub fn is_same_tree2(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                p.borrow().val == q.borrow().val
                    && Self::is_same_tree2(p.borrow().left.clone(), q.borrow().left.clone())
                    && Self::is_same_tree2(p.borrow().right.clone(), q.borrow().right.clone())
            },
            (None, None) => true,
            _ => false,
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::btree;
    use super::Solution;

    #[test]
    fn builtin() {
        assert_eq!(Solution::is_same_tree(btree![1, 2, 3], btree![1, 2, 3]), true);
        assert_eq!(Solution::is_same_tree(btree![1, 2], btree![1, null, 2]), false);
    }

    #[test]
    fn recursive() {
        assert_eq!(Solution::is_same_tree2(btree![1, 2, 3], btree![1, 2, 3]), true);
        assert_eq!(Solution::is_same_tree2(btree![1, 2], btree![1, null, 2]), false);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use crate::btree;
    use super::Solution;
    use self::test::Bencher;

    #[bench]
    fn builtin(b: &mut Bencher) {
        b.iter(|| Solution::is_same_tree(btree![1, 2, 3, 4, 5], btree![1, 2, 3, 4, 5]));
    }

    #[bench]
    fn recursive(b: &mut Bencher) {
        b.iter(|| Solution::is_same_tree2(btree![1, 2, 3, 4, 5], btree![1, 2, 3, 4, 5]));
    }
}
