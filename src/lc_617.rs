#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution;
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(root1, root2)
    }

    fn dfs(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root1.is_none() && root2.is_none() {
            return None;
        }

        let mut root = TreeNode::new(0);

        if root1.is_some() {
            root.val += root1.as_ref().unwrap().borrow().val;
        }
        if root2.is_some() {
            root.val += root2.as_ref().unwrap().borrow().val;
        }

        let (l1, l2, r1, r2): (
            Option<Rc<RefCell<TreeNode>>>,
            Option<Rc<RefCell<TreeNode>>>,
            Option<Rc<RefCell<TreeNode>>>,
            Option<Rc<RefCell<TreeNode>>>,
        );

        l1 = if root1.is_some() {
            root1.as_ref().unwrap().borrow().left.clone()
        } else {
            None
        };

        l2 = if root2.is_some() {
            root2.as_ref().unwrap().borrow().left.clone()
        } else {
            None
        };

        r1 = if root1.is_some() {
            root1.as_ref().unwrap().borrow().right.clone()
        } else {
            None
        };

        r2 = if root2.is_some() {
            root2.as_ref().unwrap().borrow().right.clone()
        } else {
            None
        };

        root.left = Self::dfs(l1, l2);
        root.right = Self::dfs(r1, r2);

        Some(Rc::new(RefCell::new(root)))
    }
}

#[test]
fn it_works() {}
