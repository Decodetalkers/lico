struct Solution;
impl Solution {
    fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len = gas.len();
        let mut start: usize = 0;
        loop {
            // start
            let mut gas_now: i32 = 0;
            let mut location = start;
            let cannot_get = loop {
                gas_now += gas[location] - cost[location];
                // 每一次到达后油肯定是大于0的，所以从这个出发点到这个点中间的点就都可以跳过了
                if gas_now < 0 {
                    break location + 1;
                }
                location += 1;
                if location == len {
                    location = 0;
                }
                if location == start {
                    return start as i32;
                }
            };
            if cannot_get <= start {
                break -1;
            }
            start = cannot_get;
        }
    }
}
fn main() {
    println!(
        "{}",
        Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
    );
    println!(
        "{}",
        Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3])
    );
}
