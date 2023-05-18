use std::mem;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    #[allow(unused)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(l1), Some(l2)) => {
            if l1.val >= l2.val {
                Some(Box::new(ListNode {
                    val: l2.val,
                    next: merge_two_lists(Some(l1), l2.next),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: l1.val,
                    next: merge_two_lists(Some(l2), l1.next),
                }))
            }
        }
    }
}
fn iter_merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut result = list1;
    let mut l2 = list2;
    let mut lsmall = &mut result;
    let lbig = &mut l2;
    while lbig.is_some() {
        if lsmall.is_none() || lsmall.as_ref()?.val > lbig.as_ref()?.val {
            mem::swap(lsmall, lbig);
        }
        if lsmall.is_none() {
            lsmall = &mut lsmall.as_mut()?.next;
        }
    }
    result
}

fn main() {}
