#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}
 
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Copy lists to be mutable
        let mut cur1 = l1;
        let mut cur2 = l2;
        // Create a new list to point to, to save the values in
        let mut dummy = Box::new(ListNode::new(0));
        let mut temp = &mut dummy;
        let mut uebertrag = 0;

        while cur1.is_some() || cur2.is_some() {
            // Take the value out of the current elements
            let node1 = cur1.as_ref().map_or(0, |n| n.val);
            let node2 = cur2.as_ref().map_or(0, |n| n.val);
            // Do modulo to get the sub 10 number
            let number = (node1 + node2 + uebertrag) % 10;
            // Get the tens
            uebertrag = (node1 + node2 + uebertrag) / 10;
            // Append the new list with the new value
            temp.next = Some(Box::new(ListNode::new(number)));
            // Go to next element
            cur1 = cur1.and_then(|n| n.next);
            cur2 = cur2.and_then(|n| n.next);
            // Go to the next new-list element
            temp = temp.next.as_mut().unwrap();
        }
        // Add the leftover tens
        if uebertrag > 0 {
            temp.next = Some(Box::new(ListNode::new(uebertrag)));
        }
        // return the head of the new list
        dummy.next
    }
}