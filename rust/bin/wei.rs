struct Solution;
impl Solution {
    // 1次 3次
    fn single_number(nums: Vec<i32>) -> i32 {
        let mut ouput: i32 = 0;
        for number in nums {
            ouput ^= number;
        }
        ouput
    }
    fn single_number2(nums: Vec<i32>) -> i32 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        for number in nums {
            b = !a & (b ^ number);
            a = !b & (a ^ number);
        }
        b
    }
}
fn main() {
    println!("{}", Solution::single_number(vec![4, 1, 2, 1, 2]));
    println!(
        "{}",
        Solution::single_number2(vec![0, 1, 0, 1, 0, 1, 99, 2, 2, 2])
    );
}
