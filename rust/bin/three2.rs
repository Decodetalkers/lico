use std::cmp::Ordering;
struct Solution;
impl Solution {
    pub fn three_sum_closest(num: Vec<i32>, target: i32) -> i32 {
        let mut num = num;
        num.sort_unstable();
        let mut output = num[0] + num[1] + num[2];
        if num.len() == 3 {
            return num[0] + num[1] + num[2];
        }
        let len = num.len();
        let mut distant = abs(output, target);
        for (i, nu) in num.iter().enumerate() {
            if i == len - 1 {
                break;
            }
            let mut m = i + 1;
            let mut n = len - 1;
            if n <= m {
                break;
            }
            let if_break = loop {
                let temp = nu + num[m] + num[n];
                match temp.cmp(&target) {
                    Ordering::Greater => {
                        let distant_temp = abs(temp, target);
                        if distant_temp < distant {
                            output = temp;
                            distant = distant_temp;
                        }
                        n -= 1;
                    }
                    Ordering::Equal => {
                        output = temp;
                        break true;
                    }
                    Ordering::Less => {
                        let distant_temp = abs(temp, target);
                        if distant_temp < distant {
                            output = temp;
                            distant = distant_temp;
                        }
                        m += 1;
                    }
                }
                if n <= m {
                    break false;
                }
            };
            if if_break {
                break;
            }
        }
        output
    }
}
fn abs(a: i32, b: i32) -> i32 {
    if a < b {
        return b - a;
    }
    a - b
}
fn main() {
    println!("{}", Solution::three_sum_closest(vec![-100,-98,-2,-1], -101));
}
