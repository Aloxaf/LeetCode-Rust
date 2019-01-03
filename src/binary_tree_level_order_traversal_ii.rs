use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut stack = VecDeque::new();
        let mut ret = vec![];
        stack.push_back(root);

        while !stack.is_empty() {
            let mut level = vec![];
            for _ in 0..stack.len() {
                let node = stack.pop_front().unwrap();
                if let Some(node) = node.as_ref().map(|n| n.borrow()) {
                    level.push(node.val);
                    stack.push_back(node.left.clone());
                    stack.push_back(node.right.clone());
                }; // 有点没明白, 总之加上分号才不会报错
            }
            if !level.is_empty() {
                ret.push(level);
            }
        }
        ret.reverse(); // 好粗暴啊
        ret
    }
}

use crate::TreeNode;
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::btree;

    #[test]
    fn test() {
        assert_eq!(
            Solution::level_order_bottom(btree![3, 9, 20, null, null, 15, 7]),
            vec![vec![15, 7], vec![9, 20], vec![3]]
        );
    }
}
