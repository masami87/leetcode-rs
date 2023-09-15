#![warn(unused_imports)]
#![warn(dead_code)]

mod lc_1186;
mod lc_1253;
mod lc_1289;
mod lc_1462;
mod lc_16;
mod lc_207;
mod lc_2178;
mod lc_2490;
mod lc_2600;
mod lc_2862;
mod lc_344;
mod lc_617;
mod lc_822;
mod lc_88;
mod lc_931;
mod lcp_50;

use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        println!("accepted!")
    }
}
