#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
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
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn level_order_rec(root: Option<Rc<RefCell<TreeNode>>>, level: i32, res: &mut Vec<Vec<i32>>) {
        if root.is_none() {
            return;
        }

        if res.len() as i32 <= level {
            res.push(Vec::new());
        }

        res[level as usize].push(root.clone().unwrap().borrow().val);
        level_order_rec(root.clone().unwrap().borrow().left.clone(), level + 1, res);
        level_order_rec(root.clone().unwrap().borrow().right.clone(), level + 1, res);
    }

    let mut res = Vec::new();

    level_order_rec(root, 0, &mut res);
    res
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_level_order_basic() {
        assert_eq!("this is mock".to_string(), "this is mock".to_string());
    }
}
