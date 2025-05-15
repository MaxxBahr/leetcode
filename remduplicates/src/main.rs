#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    // Safe head to current, extract from dummy value
    let mut current = &mut dummy.next;

    while let Some(node) = current {
        // Check if the next nodes have the same values
        while let Some(next) = node.next.as_ref() {
            if next.val == node.val {
                // Duplicate found, remove next node
                node.next = node.next.as_mut().unwrap().next.take();
            } else {
                break;
            }
        }
        current = &mut node.next;
    }

    dummy.next
}
