# LeetCode_Macros

一些方便刷题的宏

## 如何使用

两种方法

1. 把这个文件夹复制一份, 就像这个 repo 一样

2. 在你的项目的 Cargo.toml 中添加依赖 (不过会有 warning
```toml
[dependencies]
leetcode_macros = { git = "https://github.com/Aloxaf/LeetCode-Rust", path = "leetcode_macros/leetcode_macros" }
```

## Examples

初始化二叉树
```rust
use leetcode_macros::btree;

let btree = btree![1, 2, 2, null, null, 3, 3];
```

初始化链表
```rust
use leetcode_macros::linkedlist;

let linkedlist = linkedlist![1, 2, 3];
```

不知道咋描述...
```rust
use leetcode_macros::leetcode_test;

leetcode_test!(
     ["Trie", "insert", "search"]
     [[], ["apple"], ["apple"]]
     [null, null, true]
)
```

创建一个 `Vec<String>`
```rust
use leetcode_macros::vec_string;

let vec = vec_string!["1", "2", "3"];
```
