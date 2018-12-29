#![feature(proc_macro_hygiene)]

use leetcode_test::leetcode_test_debug;

#[test]
fn test() {
    println!("{}", leetcode_test_debug!(
        ["Trie","insert","search","search"]
        [[],[1, "apple"],["apple"],["app"]]
        [null,null,true,false]
    ));
}