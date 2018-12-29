use std::collections::VecDeque;

/// push O(n), pop/pop O(1) 的算法
/// 每次 push 时, 都将先前的元素逐个 pop 再 push, 这样当前元素就变成第一个元素了
///
/// 似乎还有更高效的以空间换时间的做法
/// https://cstheory.stackexchange.com/questions/2562/one-stack-two-queues/2589#2589
#[derive(Default)]
pub struct MyStack {
    queue: VecDeque<i32>,
}

/** You can modify the type of `self` for your need. */
impl MyStack {

    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }

    /** Push element x onto stack. */
    pub fn push(&mut self, x: i32) {
        self.queue.push_back(x);
        for _ in 0..self.queue.len() - 1 {
            let x = self.queue.pop_front().unwrap();
            self.queue.push_back(x);
        }
    }

    /** Removes the element on top of the stack and returns that element. */
    pub fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }

    /** Get the top element. */
    pub fn top(&self) -> i32 {
        *self.queue.front().unwrap()
    }

    /** Returns whether the stack is empty. */
    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode_test;
    use super::MyStack;

    #[test]
    fn test() {
        leetcode_test!(
            ["MyStack","push","push","top","pop","empty"]
            [[],[1],[2],[],[],[]]
            [null, null, null, 2, 2, false]
        );
    }
}
