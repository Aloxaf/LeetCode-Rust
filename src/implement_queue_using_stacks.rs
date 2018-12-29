/// 一个非常简单的思路是: pop/peek 的时候把所有元素弹出来, 取出需要的元素再 push 回去, 然而这样效率太低
/// 这个解法非常巧妙
/// https://leetcode.com/problems/implement-queue-using-stacks/discuss/64206/Short-O(1)-amortized-C%2B%2B-Java-Ruby
/// 它维护了两个栈, input 栈用于储存 push 进来的元素, output 栈用于 pop/peek, 当 output 栈为空的时候,
/// 就将 input 栈中的元素逐个弹出, 压如 output 栈中
#[derive(Default)]
pub struct MyQueue {
    input: Vec<i32>,
    output: Vec<i32>,
}

/** You can modify the type of `self` for your need. */
impl MyQueue {


    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }

    /** Push element x to the back of queue. */
    pub fn push(&mut self, x: i32) {
        self.input.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    #[inline]
    fn move_to_output(&mut self) {
        if self.output.is_empty() {
            while let Some(x) = self.input.pop() {
                self.output.push(x);
            }
        }
    }

    pub fn pop(&mut self) -> i32 {
        self.move_to_output();
        self.output.pop().unwrap()
    }

    /** Get the front element. */
    pub fn peek(&mut self) -> i32 {
        self.move_to_output();
        *self.output.last().unwrap()
    }

    /** Returns whether the queue is empty. */
    pub fn empty(&self) -> bool {
        self.input.is_empty() && self.output.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode_test;
    use super::MyQueue;

    #[test]
    fn test() {
        leetcode_test!(
            ["MyQueue","push","push","peek","pop","empty"]
            [[],[1],[2],[],[],[]]
            [null, null, null, 1, 1, false]
        );
    }
}
