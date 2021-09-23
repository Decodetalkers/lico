struct Solution;
impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut output: Vec<Vec<String>> = vec![];
        for (i, astring) in word_list.iter().enumerate() {
            let mut count = 0;
            let mut is_same = true;
            for (a, (b, c)) in begin_word
                .chars()
                .zip(astring.chars().zip(end_word.chars()))
            {
                if a != b {
                    count += 1;
                }
                if b != c {
                    is_same = false;
                }
            }
            if count == 1 {
                if is_same {
                    output.push(vec![end_word.clone()]);
                } else {
                    let mut next = word_list.clone();
                    next.remove(i);
                    let mut temp: Vec<Vec<String>> =
                        Self::find_ladders(astring.clone(), end_word.clone(), next);
                    if !temp.is_empty() {
                        output.append(&mut temp);
                    }
                }
            }
        }
        let mut output2: Vec<Vec<String>> = vec![];
        let mut minlen = 0;
        for answer in output.iter() {
            if minlen == 0 || minlen > answer.len() {
                minlen = answer.len();
                output2.clear();
            }
            if minlen == answer.len() {
                output2.push(answer.clone());
            }
        }
        if !output2.is_empty() {
            for a in output2.iter_mut() {
                a.insert(0, begin_word.clone());
            }
        }
        output2
    }
}
pub fn main() {
    println!(
        "{:?}",
        Solution::find_ladders(
            "hit".to_string(),
            "cog".to_string(),
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string()
            ]
        )
    );
    println!(
        "{:?}",
        Solution::find_ladders(
            "qa".to_string(),
            "sq".to_string(),
            vec![
                "si".to_string(),
                "go".to_string(),
                "se".to_string(),
                "cm".to_string(),
                "so".to_string(),
                "ph".to_string(),
                "mt".to_string(),
                "db".to_string(),
                "mb".to_string(),
                "sb".to_string(),
                "kr".to_string(),
                "ln".to_string(),
                "bi".to_string(),
                "er".to_string(),
                "sc".to_string(),
                "ne".to_string(),
                "mn".to_string(),
                "mi".to_string(),
                "am".to_string(),
                "ex".to_string(),
                "pt".to_string(),
                "io".to_string(),
                "be".to_string(),
                "fm".to_string(),
                "ta".to_string(),
                "tb".to_string(),
                "ni".to_string(),
                "mr".to_string(),
                "pa".to_string(),
                "he".to_string(),
                "lr".to_string(),
                "sq".to_string(),
                "ye".to_string()
            ]
        )
    );
}
