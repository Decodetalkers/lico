//Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq,Clone)]
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(ref a), Some(ref b)) => {
                let new_a = (*a).clone();
                let new_b = (*b).clone();
                if new_a.borrow().val == new_b.borrow().val {
                    Self::is_same_tree(new_a.borrow().left.clone(), new_b.borrow().left.clone())
                        && Self::is_same_tree(
                            new_a.borrow().right.clone(),
                            new_b.borrow().right.clone(),
                        )
                } else {
                    false
                }
            }
            (None, None) => true,
            (_, _) => false,
        }
    }
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(ref a) => {
                let new_root = (*a).clone();
                let next_depth = {
                    let left  = Self::max_depth(new_root.borrow().left.clone());
                    let right = Self::max_depth(new_root.borrow().right.clone());
                    if left > right {
                        left
                    } else  {
                        right
                    }
                };
                1+next_depth
            },
            None => 0,
        }
    }
}
fn main() {
    let mut a = TreeNode::new(3);
    a.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let mut b = TreeNode::new(3);
    b.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    assert!(Solution::is_same_tree(
        Some(Rc::new(RefCell::new(a.clone()))),
        Some(Rc::new(RefCell::new(b)))
    ));
    println!("{}",Solution::max_depth(Some(Rc::new(RefCell::new(a)))));
}
