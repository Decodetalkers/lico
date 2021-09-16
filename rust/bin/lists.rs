// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn new_from_vec(val: Vec<i32>) -> Self {
        let len = val.len();
        let mut list = ListNode::new(val[len - 1]);
        for i in val.iter().rev().skip(1) {
            //list.next = Some(Box::new(ListNode::new(i)));
            let temp = list.clone();
            list = ListNode {
                next: Some(Box::new(temp)),
                val: *i,
            }
        }
        list
    }
}
type List = Option<Box<ListNode>>;
trait Print {
    fn print(&self) -> String;
}
impl Print for List {
    fn print(&self) -> String {
        match &*self {
            None => "Nil".to_string(),
            Some(list) => {
                format!("{},{}", (*list).val, (*list).next.print())
            }
        }
    }
}
struct Solution;
impl Solution {
    fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> List {
        let (_, output) = get_position(head, n, true);
        output
    }
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> List {
        match (l1, l2) {
            (Some(n1), Some(n2)) => {
                if n1.val < n2.val {
                    Some(Box::new(ListNode {
                        val: n1.val,
                        next: Self::merge_two_lists(n1.next, Some(n2)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: n2.val,
                        next: Self::merge_two_lists(Some(n1), n2.next),
                    }))
                }
            }
            (Some(n1), None) => Some(n1),
            (None, Some(n2)) => Some(n2),
            _ => None,
        }
        //match l1 {
        //    None => l2,
        //    Some(ref list1)=>{
        //        match l2 {
        //            None => l1,
        //            Some(ref list2) => {
        //                if (*list1).val > (*list2).val {
        //                    let temp = from_the_end(l1.clone(), (*list2.clone()).next);
        //                    let mut test = ListNode::new((*list2).val);
        //                    test.next = temp;
        //                    return Some(Box::new(test));
        //                } else {
        //                    let temp = from_the_end((*list1.clone()).next, l2.clone());
        //                    let mut test = ListNode::new((*list1).val);
        //                    test.next = temp;
        //                    return Some(Box::new(test));
        //                }
        //            }
        //        }
        //    }
        //}
    }
}
fn get_position(head: Option<Box<ListNode>>, target: i32, is_header: bool) -> (i32, List) {
    match head {
        None => (1, None),
        Some(list) => {
            let (position, node) = get_position(list.clone().next, target, false);
            if position == target && is_header {
                return (position, node);
            }
            if position - 1 == target {
                if let Some(temp) = node {
                    let mut list = *list;
                    let temp: ListNode = *temp;
                    list.next = temp.next;
                    (position + 1, Some(Box::new(list)))
                } else {
                    (position + 1, Some(Box::new(ListNode::new(list.val))))
                }
            } else {
                let mut list = *list.clone();
                list.next = node;
                (position + 1, Some(Box::new(list)))
            }
        }
    }
}
fn main() {
    let test2 = vec![0, 1, 3, 5, 6, 7];
    let test = vec![1, 2, 3, 4, 5, 6, 7];
    let test1 = vec![1, 2, 4, 6, 10, 11];
    println!(
        "{}",
        Solution::remove_nth_from_end(Some(Box::new(ListNode::new_from_vec(test))), 2).print()
    );
    println!(
        "{}",
        Solution::merge_two_lists(
            Some(Box::new(ListNode::new_from_vec(test1))),
            Some(Box::new(ListNode::new_from_vec(test2)))
        )
        .print()
    );
}
