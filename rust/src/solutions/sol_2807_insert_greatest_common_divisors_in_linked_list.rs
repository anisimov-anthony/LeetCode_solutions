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
pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    let mut current = &mut head;
    while let Some(cur) = current {
        let a = cur.val;
        if let Some(cur_next) = &cur.next {
            let b = cur_next.val;
            let inserted = gcd(a, b);

            let mut new_node = Box::new(ListNode::new(inserted));
            new_node.next = Some(cur_next.clone());
            current.as_mut().unwrap().next = Some(new_node);
            current = &mut current.as_mut().unwrap().next.as_mut().unwrap().next;
        } else {
            current = &mut current.as_mut().unwrap().next
        }
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_greatest_common_divisors_basic() {
        let node1 = Some(Box::new(ListNode::new(3)));
        let node2 = Some(Box::new(ListNode {
            next: node1,
            val: 10,
        }));

        let node1_v = Some(Box::new(ListNode::new(3)));
        let node2_v = Some(Box::new(ListNode {
            next: node1_v,
            val: 1,
        }));
        let node3_v = Some(Box::new(ListNode {
            next: node2_v,
            val: 10,
        }));

        assert_eq!(insert_greatest_common_divisors(node2), node3_v);
    }
}
