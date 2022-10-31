// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        //create a listnode object
        let mut head = None;
        let start;
        let mut curL1 = Some(list1.unwrap().clone());
        let mut curL2 = Some(list2.unwrap().clone());

        match (curL1, curL2) {
            (None, None) => return None,
            (Some(l1), None) => {
                start = Some(l1);
            }
            (None, Some(l2)) => {
              start = Some(l2);
            }
            (Some(l1), Some(l2)) if l1.val <= l2.val => {
                curL1 = l1.next;
                start = Some(l1);
            }
            (Some(l1), Some(l2)) if l1.val > l2.val => {
              start = Some(l2);
                curL2 = l2.next;
            }
            _ => panic!("Don't know why we're hitting this arm"),
        };

        head = start.clone();

        loop {
            head = head.unwrap().next;
        }
    }
}

//main
fn main() {
    println!("Hello, world!");
}
