#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // corner case
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }

    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut prev = &mut dummy;

    while let Some(ref mut first) = prev.as_mut().unwrap().next {
        if first.next.is_none() {
            break;
        }

        let mut second = first.next.take();
        let ostatok = second.as_mut().unwrap().next.take();

        first.next = ostatok;
        second.as_mut().unwrap().next = prev.as_mut().unwrap().next.take();
        prev.as_mut().unwrap().next = second;

        prev = &mut prev.as_mut().unwrap().next.as_mut().unwrap().next;
    }

    dummy.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_to_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;
        for &val in values.iter() {
            *current = Some(Box::new(ListNode::new(val)));
            current = &mut current.as_mut().unwrap().next;
        }
        head
    }

    #[test]
    fn test_swap_pairs_basic() {
        let mut input = vec_to_list(vec![1, 2, 3, 4]);
        let mut valid_output = vec_to_list(vec![2, 1, 4, 3]);
        assert_eq!(swap_pairs(input), valid_output);

        input = vec_to_list(vec![]);
        valid_output = input.clone();
        assert_eq!(swap_pairs(input), valid_output);

        input = vec_to_list(vec![1]);
        valid_output = input.clone();
        assert_eq!(swap_pairs(input), valid_output);

        input = vec_to_list(vec![1, 2, 3]);
        valid_output = vec_to_list(vec![2, 1, 3]);
        assert_eq!(swap_pairs(input), valid_output);
    }
}
