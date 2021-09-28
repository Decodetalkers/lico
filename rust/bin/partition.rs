struct Solution;
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
                    for mut a in down.iter_mut() {
                        let mut temp = vec![temp.clone()];
                        temp.append(&mut a);
                        local.push(temp);
                    }
                }
            }
        }
        local
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
}
