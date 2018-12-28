#[derive(Default)]
pub struct MinStack {
    /// 最小元素
    min: i64,
    /// push 的元素与最小元素的差值
    stack: Vec<i64>,
}

// 在 https://leetcode.com/problems/min-stack/discuss/49031/Share-my-Java-solution-with-ONLY-ONE-stack
// 看到的解答, 只用了一个栈, 十分巧妙 (虽然这个栈占了双倍空间...

/** You can modify the type of `self` for your need. */
impl MinStack {

    /** initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }

    /// 每次都将 x - self.min 存入栈内, 并视情况更新 self.min
    pub fn push(&mut self, x: i32) {
        let x = i64::from(x);
        if self.stack.is_empty() {
            self.stack.push(0);
            self.min = x;
        } else {
            self.stack.push(x - self.min);
            if x < self.min {
                self.min = x;
            }
        }
    }

    /// 弹出一个元素, 若弹出元素小于0, 说明该元素比原先的 self.min(old) 大, 并且它是现在的最小元素
    /// 即 self.min(now) - self.min(old) = pop
    pub fn pop(&mut self) {
        if let Some(pop) = self.stack.pop() {
            if pop < 0 {
                self.min = self.min - pop;
            }
        }
    }

    /// 取出顶端元素
    pub fn top(&self) -> i32 {
        let top = *self.stack.last().unwrap();
        if top > 0 {
            (self.min + top) as i32
        } else {
            self.min as i32
        }
    }

    pub fn get_min(&self) -> i32 {
        self.min as i32
    }
}

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn test() {
        let mut obj = MinStack::new();
        obj.push(-2);
        obj.push(0);
        obj.push(-3);
        assert_eq!(obj.get_min(), -3);
        obj.pop();
        assert_eq!(obj.top(), 0);
        assert_eq!(obj.get_min(), -2);
    }
}
