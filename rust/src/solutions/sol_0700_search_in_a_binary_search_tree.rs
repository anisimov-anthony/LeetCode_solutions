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
use std::cmp::Ordering;
use std::rc::Rc;
pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut current = root;
    while let Some(ref cur) = current {
        match val.cmp(&cur.clone().borrow().val) {
            Ordering::Equal => {
                return Some(cur.clone());
            }
            Ordering::Less => {
                current = cur.clone().borrow().left.clone();
            }
            Ordering::Greater => {
                current = cur.clone().borrow().right.clone();
            }
        }
    }

    return None;
}

pub fn build_tree(data: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if data.is_empty() || data[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(data[0].unwrap())));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.clone());
    let mut i = 1;

    while !queue.is_empty() && i < data.len() {
        let current = queue.pop_front().unwrap();

        if i < data.len() {
            if let Some(val) = data[i] {
                let left_node = Rc::new(RefCell::new(TreeNode::new(val)));
                current.borrow_mut().left = Some(left_node.clone());
                queue.push_back(left_node);
            }
            i += 1;
        }

        if i < data.len() {
            if let Some(val) = data[i] {
                let right_node = Rc::new(RefCell::new(TreeNode::new(val)));
                current.borrow_mut().right = Some(right_node.clone());
                queue.push_back(right_node);
            }
            i += 1;
        }
    }

    Some(root)
}

pub fn tree_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result = Vec::new();
    let mut queue = std::collections::VecDeque::new();

    if let Some(r) = root {
        queue.push_back(r);
    } else {
        return result;
    }

    while let Some(node_ref) = queue.pop_front() {
        let node = node_ref.borrow();
        result.push(Some(node.val));

        let left = &node.left;
        let right = &node.right;

        if left.is_some() {
            queue.push_back(left.as_ref().unwrap().clone());
        } else if right.is_some() {
            result.push(None);
        }

        if let Some(right_node) = right {
            queue.push_back(right_node.clone());
        }
    }

    while result.last() == Some(&None) {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_bst_basic() {
        let data = vec![Some(4), Some(2), Some(7), Some(1), Some(3), None, None];
        let root = build_tree(&data);
        let result = search_bst(root, 2);

        assert!(result.is_some());
        assert_eq!(result.clone().unwrap().borrow().val, 2);
        assert_eq!(
            result
                .clone()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            1
        );
        assert_eq!(
            result
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            3
        );

        let data = vec![Some(4), Some(2), Some(7), Some(1), Some(3)];
        let root = build_tree(&data);
        let result = search_bst(root, 5);
        assert!(result.is_none());
    }
}
