// 最长公共前缀
struct Solution;
impl Solution {
    fn longest_common_prefix(strs: Vec<String>) -> String {
        let mini_len = get_the_min_len(&strs);
        let strs_bytes = {
            let mut bytes: Vec<Vec<u8>> = vec![];
            for astr in strs {
                bytes.push(astr.as_bytes().to_vec());
            }
            bytes
        };
        let mut finstr: Vec<u8> = vec![];
        for i in 0..mini_len {
            let header = strs_bytes[0][i];
            let res = {
                let mut check = true;
                for astr in strs_bytes.iter() {
                    if astr[i] != header {
                        check = false;
                        break;
                    }
                }
                check
            };
            if res {
                finstr.push(header);
            } else {
                break;
            }
        }
        String::from_utf8(finstr).unwrap()
    }
}

fn get_the_min_len(strs: &[String]) -> usize {
    let mut len = strs[0].len();
    for one in strs.iter() {
        if one.len() < len {
            len = one.len();
        }
    }
    len
}
fn main() {
    let test: Vec<String> = vec!["abcd".to_string(), "abcdefg".to_string(), "abc".to_string()];
    println!("{}", Solution::longest_common_prefix(test));
}
