use std::collections::HashMap;
struct Solution;
impl Solution {
    fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dict: HashMap<String, bool> = HashMap::new();
        for word in word_dict {
            dict.insert(word, true);
        }
        let len = s.len();
        let vec_x: Vec<bool> = vec![false; len];
        let mut vec_y: Vec<Vec<bool>> = vec![vec_x; len];
        for i in 0..len {
            for j in i..len {
                if dict.contains_key(&s[i..j + 1]) {
                    vec_y[j][i] = true;
                }
            }
        }

        let mut can_react = false;
        for i in 0..len {
            if vec_y[len - 1][i] {
                can_react = true;
                break;
            }
        }
        if !can_react {
            return false;
        }
        println!("{:?}", vec_y);
        //let mut position = 0;
        let mut dict2: HashMap<usize, bool> = HashMap::new();
        let mut next: Vec<usize> = vec![len - 1];
        loop {
            let mut temp: Vec<usize> = vec![];
            for a in next {
                for (i, avec) in vec_y[a].clone().into_iter().enumerate() {
                    if avec {
                        if i == 0 {
                            return true;
                        }
                        //if !dict2.contains_key(&(i-1)){
                        if let std::collections::hash_map::Entry::Vacant(e) = dict2.entry(i - 1) {
                            e.insert(true);
                            temp.push(i - 1);
                        }
                    }
                }
            }
            next = temp;
            if next.is_empty() {
                break false;
            }
        }
    }
}
fn main() {
    println!(
        "{}",
        Solution::word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        )
    );
}
