# LeetCode_Macros

一些方便刷题的宏

## Examples

初始化二叉树
```rust
let btree = btree![1, 2, 2, null, null, 3, 3];
```

初始化链表
```rust
let linkedlist = linkedlist![1, 2, 3];
```

不知道咋描述...
```rust
leetcode_test(
     ["Trie", "insert", "search"]
     [[], ["apple"], ["apple"]]
     [null, null, true]
)
```

创建一个 `Vec<String>`
```rust
let vec = vec_string!["1", "2", "3"];
```