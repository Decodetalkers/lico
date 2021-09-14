struct Status {
    forward: i32,
    maxium:i32,
}
impl Status{
    fn new() -> Status {
        Status{
            forward:0,
            maxium:0,
        }
    }
    fn max_sub_array(mut self,nums: Vec<i32>) -> i32{
        self.maxium=nums[0];
        for num in nums{
            if self.forward >=0{
                self.forward += num;
            } else if self.forward<num{
                self.forward=num;
            }
            if self.forward>self.maxium{
                self.maxium = self.forward;
            }
        }
        self.maxium
    }

}
fn main() {
    let output : Status = Status::new();
    let money: Vec<i32> = vec![-5,-61];
    println!("{}", output.max_sub_array(money));
}
