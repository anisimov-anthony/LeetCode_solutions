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
pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
    let mut bits = Vec::new();
    while let Some(ref current) = head {
        bits.push(current.val);
        head = head.unwrap().next;
    }

    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, val)| acc + val * 2_i32.pow(i as u32)) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_decimal_value_basic() {
        let node3 = Some(Box::new(ListNode::new(1)));
        let node2 = Some(Box::new(ListNode {
            val: 0,
            next: node3,
        }));
        let node1 = Some(Box::new(ListNode {
            val: 1,
            next: node2,
        }));

        assert_eq!(get_decimal_value(node1), 5);

        let node1 = Some(Box::new(ListNode::new(0)));
        assert_eq!(get_decimal_value(node1), 0);
    }
}
