#![allow(dead_code)]
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
use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code)]
pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let binding = root.unwrap();
    let root = binding.borrow();
    if root.val
        == root.left.as_ref().unwrap().borrow().val + root.right.as_ref().unwrap().borrow().val
    {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_check_tree_basic() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(10))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        root.as_ref().unwrap().borrow_mut().left = left;
        root.as_ref().unwrap().borrow_mut().right = right;
        assert!(check_tree(root));

        let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.as_ref().unwrap().borrow_mut().left = left;
        root.as_ref().unwrap().borrow_mut().right = right;
        assert!(!check_tree(root));
    }
}
