#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut slow = &head;
    let mut fast = &head;
    while let Some(ListNode {
        next: Some(node), ..
    }) = fast.as_deref()
    {
        slow = &slow.as_ref().unwrap().next;
        fast = &node.next;
    }

    slow.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vals.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut node = head;
        while let Some(n) = node {
            result.push(n.val);
            node = n.next;
        }
        result
    }

    #[test]
    fn test_odd_length() {
        let input = build_list(vec![1, 2, 3, 4, 5]);
        let output = middle_node(input);
        assert_eq!(list_to_vec(output), vec![3, 4, 5]);
    }

    #[test]
    fn test_even_length() {
        let input = build_list(vec![1, 2, 3, 4, 5, 6]);
        let output = middle_node(input);
        assert_eq!(list_to_vec(output), vec![4, 5, 6]);
    }
}
