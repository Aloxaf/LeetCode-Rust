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