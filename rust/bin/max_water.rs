mod area {
    pub struct Status {
        pub maxmun: i32,
        start: i32,
        end: i32,
    }
    impl Status {
        pub fn new(input: Vec<i32>) -> Self {
            let temp = Status {
                maxmun: 0,
                start: 0,
                end: 0,
            };
            temp.maxarea(input)
        }
        fn maxarea(mut self, input: Vec<i32>) -> Self {
            fn min(l: i32, r: i32) -> i32 {
                if l < r {
                    l
                } else {
                    r
                }
            }
            let length = input.len() as i32;
            self.end = length - 1;
            while self.end > self.start {
                let height = min(input[self.start as usize], input[self.end as usize]);
                let length = self.end - self.start;
                let area: i32 = height * length;
                if area > self.maxmun {
                    self.maxmun = area;
                }
                // if left < right, then left +1 ,else right +1
                if input[self.start as usize] <= input[self.end as usize] {
                    self.start += 1;
                } else {
                    self.end -= 1;
                }
            }
            self
        }
    }
}
use crate::area::Status;
fn main() {
    let water: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let area: Status = Status::new(water);
    println!("{}", area.maxmun);
}
