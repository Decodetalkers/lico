struct Solution;
impl Solution {
    fn letter_combinations(digits: String) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for acode in digits.chars() {
            if output.is_empty() {
                match acode {
                    '2' => {
                        output = vec!["a".to_string(), "b".to_string(), "c".to_string()];
                    }
                    '3' => {
                        output = vec!["d".to_string(), "e".to_string(), "f".to_string()];
                    }
                    '4' => {
                        output = vec!["g".to_string(), "h".to_string(), "i".to_string()];
                    }
                    '5' => {
                        output = vec!["j".to_string(), "k".to_string(), "l".to_string()];
                    }
                    '6' => {
                        output = vec!["m".to_string(), "n".to_string(), "o".to_string()];
                    }
                    '7' => {
                        output = vec![
                            "p".to_string(),
                            "q".to_string(),
                            "r".to_string(),
                            "s".to_string(),
                        ];
                    }
                    '8' => {
                        output = vec!["t".to_string(), "u".to_string(), "v".to_string()];
                    }
                    '9' => {
                        output = vec![
                            "w".to_string(),
                            "x".to_string(),
                            "y".to_string(),
                            "z".to_string(),
                        ];
                    }
                    _ => {}
                }
            } else {
                let mut temp: Vec<String> = vec![];
                match acode {
                    '2' => {
                        for acode in output.iter() {
                            let a = acode.clone() + "a";
                            let b = acode.clone() + "b";
                            let c = acode.clone() + "c";
                            temp.push(a);
                            temp.push(b);
                            temp.push(c);
                        }
                    }
                    '3' => {
                        for acode in output.iter() {
                            let d = acode.clone() + "d";
                            let e = acode.clone() + "e";
                            let f = acode.clone() + "f";
                            temp.push(d);
                            temp.push(e);
                            temp.push(f);
                        }
                    }
                    '4' => {
                        for acode in output.iter() {
                            let g = acode.clone() + "g";
                            let h = acode.clone() + "h";
                            let i = acode.clone() + "i";
                            temp.push(g);
                            temp.push(h);
                            temp.push(i);
                        }
                    }
                    '5' => {
                        for acode in output.iter() {
                            let j = acode.clone() + "j";
                            let k = acode.clone() + "k";
                            let l = acode.clone() + "l";
                            temp.push(j);
                            temp.push(k);
                            temp.push(l);
                        }
                    }
                    '6' => {
                        for acode in output.iter() {
                            let m = acode.clone() + "m";
                            let n = acode.clone() + "n";
                            let o = acode.clone() + "o";
                            temp.push(m);
                            temp.push(n);
                            temp.push(o);
                        }
                    }
                    '7' => {
                        for acode in output.iter() {
                            let p = acode.clone() + "p";
                            let q = acode.clone() + "q";
                            let r = acode.clone() + "r";
                            let s = acode.clone() + "s";
                            temp.push(p);
                            temp.push(q);
                            temp.push(r);
                            temp.push(s);
                        }
                    }
                    '8' => {
                        for acode in output.iter() {
                            let t = acode.clone() + "t";
                            let u = acode.clone() + "u";
                            let v = acode.clone() + "v";
                            temp.push(t);
                            temp.push(u);
                            temp.push(v);
                        }
                    }
                    '9' => {
                        for acode in output.iter() {
                            let w = acode.clone() + "w";
                            let x = acode.clone() + "x";
                            let y = acode.clone() + "y";
                            let z = acode.clone() + "z";
                            temp.push(w);
                            temp.push(x);
                            temp.push(y);
                            temp.push(z);
                        }
                    }
                    _ => {}
                }
                output = temp;
            }
        }
        output
    }
}
fn main() {
    println!("{:?}", Solution::letter_combinations("25".to_string()));
}
