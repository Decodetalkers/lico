struct Solution;

use std::cmp::min;
use std::collections::{hash_map::Entry::Vacant, HashMap};
impl Solution {
    fn partition(s: String) -> Vec<Vec<String>> {
        let len = s.len();
        //let mut has_result = false;
        if len == 0 {
            return vec![];
        }

        let mut local: Vec<Vec<String>> = vec![];
        for i in 0..len {
            let temp = (&s[0..i + 1]).to_string();
            if temp.is_partition() {
                let mut down = Self::partition((&s[i + 1..]).to_string());
                if down.is_empty() {
                    local.push(vec![temp.clone()]);
                } else {
                    for a in down.iter_mut() {
                        let mut temp = vec![temp.clone()];
                        temp.append(a);
                        local.push(temp);
                    }
                }
            }
        }
        local
    }
    // 棋盘啊
    fn min_cut2(s: String) -> i32 {
        let sc: Vec<char> = s.chars().collect();
        let size = s.len();
        let mut dp: Vec<Vec<bool>> = vec![vec![true; size]; size];

        // dp[i][j] = s[i] == s[j] && dp[i + 1][j - 1];
        for i in (0..size).rev() {
            for j in i + 1..size {
                dp[i][j] = sc[i] == sc[j] && dp[i + 1][j - 1];
            }
        }

        // res[i] = min(res[i], res[j] + 1) , j <= i ;
        let mut res = vec![0; size];
        for i in 0..size {
            res[i] = i as i32;
            for j in 0..=i {
                if dp[j][i] {
                    if j == 0 {
                        res[i] = 0;
                    } else {
                        res[i] = min(res[i], res[j - 1] + 1);
                    }
                }
            }
        }
        res[size - 1]
    }
    fn min_cut(s: String) -> i32 {
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        if s.is_partition() {
            return 0;
        }
        let len = s.len();
        for i in 0..len {
            for j in i..len {
                if (&s[i..j + 1]).to_string().is_partition() {
                    if let Vacant(e) = map.entry(j) {
                        e.insert(vec![i]);
                    } else {
                        let count = map.get_mut(&j).unwrap();
                        count.push(i);
                    }
                }
            }
        }
        //使用广度优先
        //println!("yes");
        fn road(map: &HashMap<usize, Vec<usize>>, location: usize) -> i32 {
            let mut output = 0;
            let mut map2: HashMap<usize, bool> = HashMap::new();
            let mut next: Vec<Vec<usize>> = vec![map.get(&location).unwrap().clone()];
            loop {
                let mut next2: Vec<Vec<usize>> = vec![];
                for ventor in next.into_iter() {
                    for avec in ventor {
                        if avec == 0 {
                            return output;
                        }
                        if map2.contains_key(&avec) {
                            break;
                        }
                        map2.insert(avec, true);
                        next2.push(map.get(&(avec - 1)).unwrap().clone());
                    }
                }
                output += 1;
                next = next2;
            }
        }
        road(&map, len - 1)
    }
}
trait Ispartition {
    fn is_partition(&self) -> bool;
}
impl Ispartition for String {
    fn is_partition(&self) -> bool {
        if self.is_empty() || self.len() == 1 {
            return true;
        }
        let test = self.clone().into_bytes();
        let mut i = 0;
        let mut j = test.len() - 1;
        loop {
            if test[i] != test[j] {
                break false;
            }
            i += 1;
            j -= 1;
            if i >= j {
                //println!("sss");
                break true;
            }
        }
        //true
    }
}
fn main() {
    println!("{:?}", Solution::partition("aabb".to_string()));
    println!("{}", Solution::min_cut("aabbaacaa".to_string()));
    println!("{}", Solution::min_cut("abbaa".to_string()));
    println!("{}", Solution::min_cut2("apjesgpsxoeiokmqmfgvjslcjukbqxpsobyhjpbgdfruqdkeiszrlmtwgfxyfostpqczidfljwfbbrflkgdvtytbgqalguewnhvvmcgxboycffopmtmhtfizxkmeftcucxpobxmelmjtuzigsxnncxpaibgpuijwhankxbplpyejxmrrjgeoevqozwdtgospohznkoyzocjlracchjqnggbfeebmuvbicbvmpuleywrpzwsihivnrwtxcukwplgtobhgxukwrdlszfaiqxwjvrgxnsveedxseeyeykarqnjrtlaliyudpacctzizcftjlunlgnfwcqqxcqikocqffsjyurzwysfjmswvhbrmshjuzsgpwyubtfbnwajuvrfhlccvfwhxfqthkcwhatktymgxostjlztwdxritygbrbibdgkezvzajizxasjnrcjwzdfvdnwwqeyumkamhzoqhnqjfzwzbixclcxqrtniznemxeahfozp".to_string()));
}
