// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            None
        } else {
            let len = nums.len();
            let mid_len = len / 2;
            let mid = nums[mid_len];
            let mut midnode = TreeNode::new(mid);
            let left_len = mid_len;
            let right_len = len - mid_len - 1;
            let left = (&nums[0..mid_len]).to_vec();
            let right = (&nums[mid_len + 1..]).to_vec();
            match (left_len, right_len) {
                (0, 0) => {}
                (_, 0) => {
                    let left_node = Self::sorted_array_to_bst(left);
                    midnode.left = left_node;
                }
                (0, _) => {
                    let right_node = Self::sorted_array_to_bst(right);
                    midnode.right = right_node;
                }
                (_, _) => {
                    let left_node = Self::sorted_array_to_bst(left);
                    let right_node = Self::sorted_array_to_bst(right);
                    midnode.left = left_node;
                    midnode.right = right_node;
                }
            }
            Some(Rc::new(RefCell::new(midnode)))
        }
    }
}
type List = Option<Rc<RefCell<TreeNode>>>;
trait Print {
    fn print(&self) -> String;
}
impl Print for List {
    fn print(&self) -> String {
        match self {
            None => String::new(),
            Some(tree) => {
                let temp = (*tree).borrow();
                match (&temp.left, &temp.right) {
                    (None, None) => format!("{}", temp.val),
                    (None, Some(_)) => format!("{},[{}]", temp.val, temp.right.print()),
                    (Some(_), None) => format!("[{}],{}", temp.left.print(), temp.val),
                    (Some(_), Some(_)) => format!(
                        "[{}],{},[{}]",
                        temp.left.print(),
                        temp.val,
                        temp.right.print()
                    ),
                }
            }
        }
    }
}
pub fn main() {
    println!(
        "{}",
        Solution::sorted_array_to_bst(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]).print()
    );
}
