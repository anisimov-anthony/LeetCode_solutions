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
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });

    let mut fast = dummy.clone();
    let mut slow = &mut dummy;

    for _ in 0..n {
        fast = fast.next.unwrap();
    }

    while fast.next.is_some() {
        fast = fast.next.unwrap();
        slow = slow.next.as_mut().unwrap();
    }

    slow.next = slow.next.as_mut().unwrap().next.take();

    dummy.next
}

#[allow(dead_code)]
fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;

    for val in values {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }

    dummy.next
}

#[allow(dead_code)]
fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;

    while let Some(node) = current {
        result.push(node.val);
        current = node.next;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let head = create_list(vec![1, 2, 3, 4, 5]);
        let result = remove_nth_from_end(head, 2);
        assert_eq!(list_to_vec(result), vec![1, 2, 3, 5]);

        let head = create_list(vec![1]);
        let result = remove_nth_from_end(head, 1);
        assert_eq!(list_to_vec(result), vec![]);

        let head = create_list(vec![1, 2]);
        let result = remove_nth_from_end(head, 1);
        assert_eq!(list_to_vec(result), vec![1]);

        let head = create_list(vec![1, 2, 3]);
        let result = remove_nth_from_end(head, 3);
        assert_eq!(list_to_vec(result), vec![2, 3]);

        let head = create_list(vec![1, 2, 3]);
        let result = remove_nth_from_end(head, 1);
        assert_eq!(list_to_vec(result), vec![1, 2]);
    }
}
