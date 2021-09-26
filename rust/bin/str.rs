struct Solution;
impl Solution {
    fn str_str(haystack: String, needle: String) -> i32 {
        let hay = haystack.into_bytes();
        let need = needle.into_bytes();
        let len1 = hay.len();
        let len2 = need.len();
        if len2 == 0 {
            return 0;
        } else if len1 < len2 {
            return -1;
        }
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut position;
        let mut is_break;
        loop {
            if hay[i] == need[0] {
                position = i as i32;
                i += 1;
                j += 1;
                is_break = loop {
                    if j == len2 {
                        break true;
                    } else if i == len1 || hay[i] != need[j] {
                        break false;
                    } else {
                        i += 1;
                        j += 1;
                    }
                };
                if is_break {
                    break position;
                } else if i == len1 || j == len2 {
                    break -1;
                } else {
                    i = position as usize + 1;
                    j = 0;
                    continue;
                }
            } else {
                i += 1;
                if i == len1 {
                    break -1;
                }
            }
        }
    }
}
fn main() {
    println!(
        "{}",
        Solution::str_str("mississippi".to_string(), "issip".to_string())
    );
}
