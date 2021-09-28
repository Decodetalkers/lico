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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut output = 0;
        get_output(root, &mut output);
        output
    }
}
fn get_output(root: Option<Rc<RefCell<TreeNode>>>, output: &mut i32) {
    if let Some(ref tree) = root {
        *output *= 10;
        let tep = (*tree).borrow();
        let val = tep.val;
        *output += val;
        match (tep.left.clone(), tep.right.clone()) {
            (Some(_), Some(_)) => {
                let mut temp: i32 = *output;
                get_output(tep.left.clone(), &mut (*output));
                get_output(tep.right.clone(), &mut temp);
                *output += temp;
            }
            (Some(_), None) => {
                get_output(tep.left.clone(), &mut *output);
            }
            (None, Some(_)) => {
                get_output(tep.right.clone(), &mut *output);
            }
            (None, None) => {}
        }
    }
}
pub fn main() {
    //let mut list = TreeNode::new(4);
    let list_left = TreeNode {
        val: 9,
        left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
    };
    let list_right = TreeNode::new(0);
    let list = TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(list_left))),
        right: Some(Rc::new(RefCell::new(list_right))),
    };
    println!(
        "{}",
        Solution::sum_numbers(Some(Rc::new(RefCell::new(list))))
    );
}
