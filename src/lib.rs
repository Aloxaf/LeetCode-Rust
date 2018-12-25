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
            let mut head = Box::new(ListNode::new(0));
            let mut ref_head = &mut head;

            $(
            ref_head.next = Some(Box::new(ListNode::new($e)));
            ref_head = ref_head.next.as_mut().unwrap();
            )*

            let _ = ref_head; // 避免 `unused_assignments`
            head.next
        }
    }
}


// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
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