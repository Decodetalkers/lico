use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        //let mut map2 : HashMap<i32,i32> = HashMap::new();
        for num in nums.iter() {
            map.insert(*num, 0);
        }
        let mut longest = 0;
        for amap in map.keys() {
            if map.contains_key(&(amap - 1)) {
                continue;
            } else {
                let mut long_temp = 1;
                let mut j = 1;
                while map.contains_key(&(amap + j)) {
                    long_temp += 1;
                    j += 1;
                }
                if long_temp > longest {
                    longest = long_temp;
                }
            }
        }
        longest
    }
}
pub fn main() {
    println!(
        "{}",
        Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2])
    );
}
