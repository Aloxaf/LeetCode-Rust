use std::rc::Rc;
use std::cell::RefCell;

/// Definition for a binary tree node.
/// 这个地方为了方便调试时的排序, 实现了 Ord 和 PartialOrd
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// 想了想, 其实不要这个宏也行
// 用 stringfiy! 全转成字符串就可以了...
#[macro_export]
macro_rules! null_to_none {
    (@start $($e:tt), *) => {
        {
            let mut ret: Vec<Option<i32>> = vec![];
            $crate::null_to_none![@next ret; $($e), *];
            ret
        }
    };
    (@next $vec:expr; null, $($tail:tt), *) => {
        $vec.push(None);
        $crate::null_to_none![@next $vec; $($tail), *];
    };
    (@next $vec:expr; $e:tt, $($tail:tt), *) => {
        $vec.push(Some($e));
        $crate::null_to_none![@next $vec; $($tail), *];
    };
    (@next $vec:expr; null) => {
        $vec.push(None);
    };
    (@next $vec:expr; $e:tt) => {
        $vec.push(Some($e));
    };
    ($($e:tt), *) => {
        $crate::null_to_none![@start $($e), *]
    };
}

/// Create a binary tree with TreeNode
///
/// # Example
///
/// ```rust
/// let tree = btree![1, 2, 3, null, null, 4, 5]
/// ```
#[macro_export]
macro_rules! btree {
    () => {
        None
    };
    ($($e:tt), *) => {
        {
            let elems = $crate::null_to_none![$($e), *];
            let head = Some($crate::Rc::new($crate::RefCell::new($crate::TreeNode::new(elems[0].unwrap()))));
            let mut nodes = std::collections::VecDeque::new();
            nodes.push_back(head.as_ref().unwrap().clone());

            for i in elems[1..].chunks(2) {
                let node = nodes.pop_front().unwrap();
                if let Some(val) = i[0]{
                    node.borrow_mut().left = Some($crate::Rc::new($crate::RefCell::new($crate::TreeNode::new(val))));
                    nodes.push_back(node.borrow().left.as_ref().unwrap().clone());
                }
                if i.len() > 1 {
                    if let Some(val) = i[1] {
                        node.borrow_mut().right = Some($crate::Rc::new($crate::RefCell::new($crate::TreeNode::new(val))));
                        nodes.push_back(node.borrow().right.as_ref().unwrap().clone());
                    }
                }
            }
            head
        }
    };
}