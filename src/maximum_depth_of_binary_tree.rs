use crate::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => 1 + Solution::max_depth(node.borrow().left.clone()).max(Solution::max_depth(node.borrow().right.clone())),
            None => 0,
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::btree;

    #[test]
    fn test() {
        assert_eq!(Solution::max_depth(btree![3, 9, 20, null, null, 15, 7]), 3);
    }
}