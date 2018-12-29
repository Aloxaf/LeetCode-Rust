use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_mirror(root.clone(), root)
    }

    fn is_mirror(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (t1, t2) {
            (Some(t1), Some(t2)) => {
                t1.borrow().val == t2.borrow().val
                    && Self::is_mirror(t1.borrow().left.clone(), t2.borrow().right.clone())
                    && Self::is_mirror(t1.borrow().right.clone(), t2.borrow().left.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }

    pub fn is_symmetric_iter(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = vec![];
        queue.push(root.clone());
        queue.push(root);

        while !queue.is_empty() {
            match (queue.pop(), queue.pop()) {
                (Some(Some(t1)), Some(Some(t2))) => {
                    if t1.borrow().val != t2.borrow().val {
                        return false;
                    }
                    queue.push(t1.borrow().left.clone());
                    queue.push(t2.borrow().right.clone());
                    queue.push(t1.borrow().right.clone());
                    queue.push(t2.borrow().left.clone());
                },
                (Some(None), Some(None)) => (),
                _ => return false,
            }
        }
        true
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::btree;

    #[test]
    fn recursive() {
        assert_eq!(Solution::is_symmetric(btree![1, 2, 2, 3, 4, 4, 3]), true);
        assert_eq!(
            Solution::is_symmetric(btree![1, 2, 2, null, 3, null, 3]),
            false
        );
    }

    #[test]
    fn iter() {
        assert_eq!(Solution::is_symmetric_iter(btree![1, 2, 2, 3, 4, 4, 3]), true);
        assert_eq!(
            Solution::is_symmetric_iter(btree![1, 2, 2, null, 3, null, 3]),
            false
        );
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use crate::btree;
    use super::Solution;
    use self::test::Bencher;

    #[bench]
    fn recursive(b: &mut Bencher) {
        b.iter(|| Solution::is_symmetric(btree![1, 2, 2, 3, 4, 4, 3, 1, 2, 2, 1, 1, 2, 2, 1]));
    }

    #[bench]
    fn iter(b: &mut Bencher) {
        b.iter(|| Solution::is_symmetric_iter(btree![1, 2, 2, 3, 4, 4, 3, 1, 2, 2, 1, 1, 2, 2, 1]));
    }
}
