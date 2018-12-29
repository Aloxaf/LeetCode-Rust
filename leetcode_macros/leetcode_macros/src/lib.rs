#![feature(uniform_paths)]

mod btree;
mod linkedlist;

pub use btree::TreeNode;
pub use linkedlist::ListNode;
pub use leetcode_test::leetcode_test;

pub use std::rc::Rc;
pub use std::cell::RefCell;

#[macro_export]
macro_rules! vec_string {
    ($($e:expr), *) => {vec![$($e.to_owned()), *]};
}