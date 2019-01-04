use std::rc::Rc;
use std::cell::RefCell;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    /// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/discuss/36977/My-short-post-order-traversal-Java-solution-for-share
    /// NB, 相形见绌
    /// 我佛了
    pub fn flatten(root: &mut Node) {
        fn core(node: &mut Node, mut next: Node) -> Node {
            if let Some(node) = node {
                next = core(&mut node.borrow_mut().right, next);
                next = core(&mut node.borrow_mut().left, next);
                node.borrow_mut().right = next;
                node.borrow_mut().left = None;
                next = Some(node.clone());
            }
            next
        }
        core(root, None);
    }
}

pub struct Solution;
use crate::TreeNode;

#[cfg(test)]
mod tests {
    use crate::btree;
    use super::Solution;

    #[test]
    fn test() {
        let mut v = btree![1, 2, 5, 3, 4, null, 6];
        Solution::flatten(&mut v);
        assert_eq!(v, btree![1, null, 2, null, 3, null, 4, null, 5, null, 6]);
    }
}