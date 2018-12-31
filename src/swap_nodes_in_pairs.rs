use std::mem;

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut n1 = &mut head;
        while n1.is_some() && n1.as_ref().unwrap().next.is_some() {
            // 冷静分析一下这个问题, 假设结构是这样的 n1 -> n2 -> ... , cur 是 &mut n1
            // 要想交换 n1, n2, 首先将 n2 从 n1.next 移出, 以实现同时拥有 n1, n2 的可变借用
            // 那么就会得到 n1 -> None, n2 -> ...
            // 再将 n1.next 与 n2.next 交换: n1 -> ..., n2 -> None
            // 最后将 n1 与 n2.next 交换: n2 -> n1 -> ...;
            let mut n2 = None;
            mem::swap(&mut n1.as_mut().unwrap().next, &mut n2);
            mem::swap(&mut n1.as_mut().unwrap().next, &mut n2.as_mut().unwrap().next);
            mem::swap(n1, &mut n2.as_mut().unwrap().next);
            // n2 是临时变量, 得将里面的内容移出来
            mem::swap(n1, &mut n2);
            n1 = &mut n1.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        head
    }
}

// 为啥我要放在后面呢, 因为方便复制代码
use crate::ListNode;
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn test() {
        assert_eq!(
            Solution::swap_pairs(linkedlist![1, 2, 3, 4]),
            linkedlist![2, 1, 4, 3]
        );
    }
}
