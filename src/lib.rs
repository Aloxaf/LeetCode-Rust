#![cfg_attr(feature = "bench", feature(test))]

use std::rc::Rc;
use std::cell::RefCell;

pub mod two_sum;
pub mod add_two_numbers;
pub mod longest_substring_without_repeating_characters;
pub mod reverse_integer;
pub mod palindrome_number;
pub mod roman_to_integer;
pub mod valid_parentheses;
pub mod remove_duplicates_from_sorted_array;
pub mod merge_two_sorted_lists;
pub mod remove_element;
pub mod implement_strstr;
pub mod sqrtx;
pub mod all_possible_full_binary_trees;
pub mod counting_bits;
pub mod longest_common_prefix;
pub mod search_insert_position;
pub mod string_to_integer_atoi;
pub mod evaluate_reverse_polish_notation;
pub mod valid_sudoku;
pub mod count_and_say;
pub mod longest_palindromic_substring;
pub mod length_of_last_word;
pub mod integer_to_roman;
pub mod group_anagrams;
pub mod maximum_subarray;
pub mod rotate_image;
pub mod remove_nth_node_from_end_of_list;
pub mod climbing_stairs;
pub mod power_of_two;
pub mod plus_one;
pub mod add_binary;
pub mod valid_palindrome;
pub mod jewels_and_stones;
pub mod to_lower_case;
pub mod unique_morse_code_words;
pub mod maximum_depth_of_binary_tree;
pub mod merge_sorted_array;
pub mod pascals_triangle;
pub mod count_primes;
pub mod isomorphic_strings;
pub mod basic_calculator;
pub mod largest_number;
pub mod non_decreasing_array;
pub mod generate_parentheses;
pub mod excel_sheet_column_number;
pub mod rotate_array;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[macro_export]
macro_rules! linkedlist {
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            let mut head = Box::new($crate::ListNode::new(0));
            let mut ref_head = &mut head;

            $(
            ref_head.next = Some(Box::new($crate::ListNode::new($e)));
            ref_head = ref_head.next.as_mut().unwrap();
            )*

            let _ = ref_head; // 避免 `unused_assignments`
            head.next
        }
    };
}


// Definition for a binary tree node.
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
// 用 vec_string 全转成字符串就可以了...
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

// 二叉树层序遍历初始化宏
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

#[macro_export]
macro_rules! vec_string {
    ($($e:expr), *) => {vec![$($e.to_owned()), *]};
}