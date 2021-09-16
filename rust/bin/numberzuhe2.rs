use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut map = HashMap::new();
        map.insert('2', "abc".to_string());
        map.insert('3', "def".to_string());
        map.insert('4', "ghi".to_string());
        map.insert('5', "jkl".to_string());
        map.insert('6', "mno".to_string());
        map.insert('7', "pqrs".to_string());
        map.insert('8', "tuv".to_string());
        map.insert('9', "wxyz".to_string());

        fn dfs(
            idx: usize,
            digits: &str,
            map: &HashMap<char, String>,
            path: &mut String,
            ans: &mut Vec<String>,
        ) {
            if digits.len() == idx {
                ans.push(path.clone());
                return;
            }
            let cc = digits.chars().nth(idx).unwrap();
            for c in map.get(&cc).unwrap().chars() {
                path.push(c);
                dfs(idx + 1, digits, map, path, ans);
                // 回溯，pop之前的东西
                path.pop();
            }
        }
        let mut ans = Vec::new();
        if digits.is_empty() {
            return ans;
        }
        dfs(0, &digits, &map, &mut String::new(), &mut ans);
        ans
    }
}
// 回溯
fn main() {
    println!("{:?}", Solution::letter_combinations("255".to_string()));
}
