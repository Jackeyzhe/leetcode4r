// Definition for singly-linked list.
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

struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut prev: Option<Box<ListNode>> = None;
        let mut current: Option<Box<ListNode>> = head;
        while current.is_some() {
            let mut node = current.take().unwrap();
            current = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

fn main() {
    let mut node5 = ListNode::new(5);
    let mut node4 = ListNode::new(4);
    let mut node3 = ListNode::new(3);
    let mut node2 = ListNode::new(2);
    let mut node1 = ListNode::new(1);
    node4.next = Some(Box::new(node5));
    node3.next = Some(Box::new(node4));
    node2.next = Some(Box::new(node3));
    node1.next = Some(Box::new(node2));
    let mut node = Some(Box::new(node1));
    let mut revert = Solution::reverse_list(node);
    while revert.is_some() {
        let mut n = revert.take().unwrap();
        println!("{}", n.val);
        revert = n.next;
    }
}