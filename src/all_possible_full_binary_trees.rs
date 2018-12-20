use crate::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

// 大概是 N 的值比较微妙? 加上缓存之后速度反而下降了
// 其实看到带缓存版的一堆 clone 我就觉得速度应该会下降(
impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ret = vec![];
        if n % 2 == 0 {
            ;
        } else if n == 1 {
            ret.push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
        } else {
            for x in (1..n).step_by(2) {
                for left in Solution::all_possible_fbt(x) {
                    for right in Solution::all_possible_fbt(n - 1 - x) {
                        let mut bns = TreeNode::new(0);
                        bns.left = left.clone();
                        bns.right = right.clone();
                        ret.push(Some(Rc::new(RefCell::new(bns))));
                    }
                }
            }
        }
        ret
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        // TODO: 啊这个 test 写起来感觉很复杂的样子
        unimplemented!()
    }
}