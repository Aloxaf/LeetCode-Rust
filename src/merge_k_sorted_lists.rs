use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::mem;

//这道题提交的时候要提交下面的代码, 因为 LeetCode 的 ListNode 并没有自动 derive Ord + PartialOrd
//use std::cmp::Ordering;
//impl Ord for ListNode {
//    fn cmp(&self, other: &ListNode) -> Ordering {
//        self.val.cmp(&other.val)
//    }
//}
//
//impl PartialOrd for ListNode {
//    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
//        Some(self.val.cmp(&other.val))
//    }
//}

impl Solution {
    /// 基本思路, 存 Vec 里排序
    /// 本来准备用优先队列, 想了想感觉也没简单多少?
    /// 后来我还是用了优先队列, 因为发现用 std::cmp::Reverse 可以方便地逆转大小关系
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut nodes = BinaryHeap::new();

        for head in lists.iter_mut() {
            let head = head;
            while head.is_some() {
                let mut tmp = None;
                // 首先, 将 head 指向的节点移至 tmp,
                mem::swap(&mut tmp, head);
                // 然后, 将 head 更新为 tmp.next 指向的节点
                mem::swap(head, &mut tmp.as_mut().unwrap().next);
                // 最后, 将这个独立的节点压入 nodes
                nodes.push(Reverse(tmp));
            }
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = &mut dummy;

        while let Some(node) = nodes.pop() {
            cur.next = node.0;
            cur = cur.next.as_mut().unwrap();
        }

        dummy.next
    }
}

use crate::ListNode;
pub struct Solution;

#[cfg(test)]
mod tests {
    use crate::linkedlist;
    use super::{Solution};

//    macro_rules! linkedlist {
//        () => {
//            None
//        };
//        ($($e:expr), *) => {
//            {
//                let mut head = Box::new(ListNode::new(0));
//                let mut ref_head = &mut head;
//
//                $(
//                ref_head.next = Some(Box::new(ListNode::new($e)));
//                ref_head = ref_head.next.as_mut().unwrap();
//                )*
//
//                let _ = ref_head; // 避免 `unused_assignments`
//                head.next
//            }
//        };
//    }

    #[test]
    fn test() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                linkedlist![1, 4, 5],
                linkedlist![1, 3, 4],
                linkedlist![2, 6],
            ]),
            linkedlist![1, 1, 2, 3, 4, 4, 5, 6]
        );
    }
}
