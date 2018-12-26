# LeetCode-Rust
听说 LeetCode 添加了 Rust 支持, 这岂不是双倍的快(fu)乐(za)?   
于是来体验一下

用 Rust 写题真是酸爽无比啊! (各种意义上的)

## 刷了几天的感受
Rust 运行速度是真 TM 快

有了 NLL 以后与 borrow checker 作斗争的次数也少了, 体验大幅上升

<s>是刷题的不二之选</s>

<s>以上不包括使用了非标准库的链表、树等数据结构的题目</s>

## 常用命令

### 测试
```
cargo test --lib add_binary -- --nocapture
```

### Benchmark
```
cargo bench --lib length_of_last_word --features bench -- --nocapture
```
