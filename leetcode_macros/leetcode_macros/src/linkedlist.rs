/// Definition for singly-linked list.
/// 这个地方为了方便调试时的排序, 实现了 Ord 和 PartialOrd
#[derive(PartialEq, Eq, Debug, Ord, PartialOrd)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Create a linked list with ListNode
///
/// # Example
///
/// ```rust
/// let list = linkedlist![1, 2, 3]
/// ```
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