use std::collections::HashMap;
struct Solution;
impl Solution {
    fn three_sum(sums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut output: Vec<Vec<i32>> = vec![];
        if sums.len() >= 3 {
            let mut map: HashMap<i32, i32> = HashMap::new();
            let mut copy: Vec<i32> = vec![];
            let mut map_two: HashMap<i32, i32> = HashMap::new();
            for acode in sums.iter() {
                let count = map.entry(*acode).or_insert(0);
                *count += 1;
                if *count == 1 {
                    copy.push(*acode);
                }
                if *acode != 0 && *count == 2 {
                    map_two.insert(*acode, 1);
                }
                if *acode == 0 && *count == 3 {
                    output.push(vec![0, 0, 0]);
                }
            }
            for (key, _) in map_two {
                let code = -2 * key;
                if map.contains_key(&code) {
                    output.push(vec![key, key, code]);
                }
            }
            let len = copy.len();
            copy.sort_unstable();
            for i in 0..len {
                if copy[i] > 0 {
                    break;
                }
                for j in i + 1..len {
                    let code = -(copy[i] + copy[j]);
                    if code > copy[j] && map.contains_key(&code) {
                        output.push(vec![copy[i], copy[j], code]);
                    }
                }
            }
            //let after_sort = sums.clone().sort();
        }
        output
    }
}
fn main() {
    println!("{:?}", Solution::three_sum(vec![3, 0, -2, -1, 1, 2]));
}
