struct Solution;
enum Positon {
    Left,
    Right,
}
impl Positon {
    fn get_positon(s: char) -> Self {
        if s == '(' || s == '[' || s == '{' {
            Self::Left
        } else {
            Self::Right
        }
    }
}

impl Solution {
    fn is_valid(s: String) -> bool {
        fn is_pair(left: char, right: char) -> bool {
            (left == '(' && right == ')')
                || (left == '[' && right == ']')
                || (left == '{' && right == '}')
        }
        let mut temp: Vec<char> = vec![];
        for achar in s.chars() {
            let positon = Positon::get_positon(achar);
            match positon {
                Positon::Left => {
                    temp.push(achar);
                }
                Positon::Right => {
                    if temp.is_empty() {
                        return false;
                    }
                    if is_pair(temp[temp.len() - 1], achar) {
                        temp.pop();
                    } else {
                        return false;
                    }
                }
            }
        }
        temp.is_empty()
    }
}
fn main() {
    assert!(Solution::is_valid("([(])".to_string()));
}
