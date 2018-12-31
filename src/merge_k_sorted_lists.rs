use crate::ListNode;

use std::mem;

//#[derive(PartialEq, Eq, Debug)]
//pub struct ListNode {
//    pub val: i32,
//    pub next: Option<Box<ListNode>>,
//}
//
//impl ListNode {
//    #[inline]
//    pub fn new(val: i32) -> Self {
//        ListNode { next: None, val }
//    }
//}
//
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
//        Some(self.cmp(other))
//    }
//}

impl Solution {
    /// 基本思路, 存 Vec 里排序
    /// 本来准备用优先队列, 想了想感觉也没简单多少?
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut nodes = vec![];

        for head in lists.iter_mut() {
            let head = head;
            while head.is_some() {
                let mut tmp = None;
                // 首先, 将 head 指向的节点移至 tmp,
                mem::swap(&mut tmp, head);
                // 然后, 将 head 更新为 tmp.next 指向的节点
                mem::swap(head, &mut tmp.as_mut().unwrap().next);
                // 最后, 将这个独立的节点压入 nodes
                nodes.push(tmp);
            }
        }
        nodes.sort_unstable();

        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = &mut dummy;

        for node in nodes {
            cur.next = node;
            cur = cur.next.as_mut().unwrap();
        }

        dummy.next
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::linkedlist;

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
