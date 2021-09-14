struct Solution;
impl Solution {
    fn int_to_roman(num:i32) -> String {
        //let mut record = num;
        if num < 1000 {
            int_to_roman_underthound(num,false)
        } else {
            int_to_roman_overthound(num,false)
        }
    }
}
fn int_to_roman_under_ten(num:i32, one: char, five: char, ten: char) -> String {
    if num == 0{
        "".to_string()
    } else if num < 5 {
        if num == 4 {
            format!("{}{}",one,five)
        } else {
            let mut i_collection = String::new();
            for _ in 0..num {
                i_collection = format!("{}{}",i_collection,one);
            }
            i_collection
        }
    } else if num == 9 {
        format!("{}{}",one,ten)
    } else {
        let mut i_collection = five.to_string();
        for _ in 5..num {
            i_collection = format!("{}{}",i_collection,one);
        }
        i_collection
    }
}

fn int_to_roman_underthound(num:i32,thound_header: bool) -> String {
    let over_hundred = num /100;
    let hundred = int_to_roman_under_ten(over_hundred,'C','D','M');
    let over_ten = (num % 100)/10;
    let ten = int_to_roman_under_ten(over_ten,'X','L','C');
    let over_zero = num % 10;
    let one  = {
        if thound_header{
            int_to_roman_under_ten(over_zero,'M','V','X')
        } else {
            int_to_roman_under_ten(over_zero,'I','V','X')
        }
    };
    format!("{}{}{}",hundred,ten,one)
}
fn int_to_roman_overthound(num:i32,thound_header: bool)-> String{
    let under_thound = num % 1000;
    let header = num / 1000;
    if header > 1000 {
        format!("{}{}", int_to_roman_overthound(header,true),int_to_roman_underthound(under_thound,thound_header))
    } else {
        format!("{}{}",int_to_roman_underthound(header,true),int_to_roman_underthound(under_thound,thound_header))
    }
}
fn main() {
    let a = Solution::int_to_roman(2000);
    println!("{}",a);
    
}
