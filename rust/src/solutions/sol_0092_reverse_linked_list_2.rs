#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    // corner case
    if head.is_none() || left == right {
        return head;
    }

    // main case
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut before = &mut dummy;

    for _ in 1..left {
        before = &mut before.as_mut()?.next;
    }

    let mut rev_head = before.as_mut()?.next.take();
    let mut curr = rev_head.as_mut()?.next.take();

    for _ in left..right {
        let next = curr.as_mut()?.next.take();
        curr.as_mut()?.next = rev_head;
        rev_head = curr;
        curr = next;
    }

    before.as_mut()?.next = rev_head;
    while before.as_ref()?.next.is_some() {
        before = &mut before.as_mut()?.next;
    }
    before.as_mut()?.next = curr;

    dummy?.next
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_between_basic() {
        let mut input = create_list(vec![1, 2, 3, 4, 5]);
        let mut valid_output = create_list(vec![1, 4, 3, 2, 5]);
        assert_eq!(reverse_between(input, 2, 4), valid_output);

        input = create_list(vec![5]);
        valid_output = create_list(vec![5]);
        assert_eq!(reverse_between(input, 1, 1), valid_output);
    }
}
