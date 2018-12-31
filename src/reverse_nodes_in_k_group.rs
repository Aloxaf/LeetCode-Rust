use std::mem;

impl Solution {
    /// https://leetcode.com/problems/reverse-nodes-in-k-group/discuss/11423/Short-but-recursive-Java-code-with-comments
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        let mut cnt = 0;
        while cur.is_some() && cnt != k {
            cur = &mut cur.as_mut().unwrap().next;
            cnt += 1;
        }
        if cnt == k {
            // 取出余下的链表
            let tail = mem::replace(cur, None);
            // 对余下的链表进行翻转
            let mut tail = Self::reverse_k_group(tail, k);
            cur = &mut tail;
            // 以这个链表为例 1->2->3->4->5, 翻转后的结果应该是 3->2->1->4->5
            // 此时的情况为 tail = 4->5, head = 1->2->3, cur = &mut tail
            // 第一步: 将 head.next 与 tail 交换: head = 1->4->5, tail = 2->3
            // 第二步: 将 head 与 tail 交换: tail = 1->4->5, head = 2->3
            // 重复 k - 1 次
            for _ in 0..k {
                mem::swap(&mut head.as_mut().unwrap().next, cur);
                mem::swap(&mut head, cur);
            }
            // 恢复 head 的指向, 毕竟最终返回的是 head
            mem::swap(&mut head, cur);
        }
        head
    }
}

use crate::ListNode;
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn test() {
        assert_eq!(
            Solution::reverse_k_group(linkedlist![1, 2, 3, 4, 5], 2),
            linkedlist![2, 1, 4, 3, 5],
        );

        assert_eq!(
            Solution::reverse_k_group(linkedlist![1, 2, 3, 4, 5], 3),
            linkedlist![3, 2, 1, 4, 5],
        );
    }
}