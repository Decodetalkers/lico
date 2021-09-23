struct Solution;
impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut before = word_list.clone();
        for (i, astring) in word_list.iter().enumerate() {
            if **astring == begin_word {
                before.remove(i);
            }
        }
        find_ladders_before(vec![begin_word.clone()], end_word, vec![before], vec![]).0
    }
}
fn find_ladders_before(
    begin_words: Vec<String>,
    end_word: String,
    word_lists: Vec<Vec<String>>,
    see_before: Vec<String>,
) -> (Vec<Vec<String>>, Vec<i32>) {
    let mut counts = see_before.clone();
    let mut astrings = vec![];
    let mut nexts = vec![];
    let mut output: Vec<Vec<String>> = vec![];
    //let mut begin_world_record = vec![];
    // 记录分裂次数
    let mut local_split: Vec<i32> = vec![];
    //let mut zero: i32 = 0;
    //let mut upper_split:Vec<i32> = vec![];
    for (word_list, begin_word) in word_lists.iter().zip(begin_words.iter()) {
        let mut show_count = 0;
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
                let mut have_seen = false;
                for j in &see_before {
                    if *astring == **j && !is_same {
                        have_seen = true;
                    }
                }
                if !have_seen {
                    show_count += 1;
                    //begin_world_record.push(begin_word.clone());
                    if is_same {
                        output.push(vec![end_word.clone()]);
                    } else {
                        counts.push(astring.clone());
                        let mut next = word_list.clone();
                        next.remove(i);
                        astrings.push(astring.clone());
                        nexts.push(next);
                    }
                }
            }
        }
        local_split.push(show_count);
    }

    //for (astring, next) in astrings.iter().zip(nexts.into_iter()) {
    let mut temp = vec![];
    let mut down_split: Vec<i32> = vec![];
    if !astrings.is_empty() {
        let a = find_ladders_before(astrings, end_word.clone(), nexts, counts.clone());
        temp = a.0;
        down_split = a.1;
    }
    let mut grand_split: Vec<i32> = local_split.clone();
    println!("{:?},{:?}", begin_words, word_lists);
    println!("{:?},{:?}", down_split, local_split);
    if !down_split.is_empty() {
        grand_split.clear();
        let mut position = 0;
        for a in local_split.iter() {
            let mut len = 0;
            for i in position..position + a {
                len += down_split[i as usize];
            }
            position += a;
            grand_split.push(len);
        }
    }
    if !temp.is_empty() {
        output.append(&mut temp);
    }
    //}
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
    let mut begin_world_record: Vec<String> = vec![];
    for (word, count) in begin_words.iter().zip(grand_split.clone()) {
        for _ in 0..count {
            begin_world_record.push(word.clone());
        }
    }
    //let mut output3 =  vec![];
    if !output2.is_empty() {
        //for begin_word in begin_words.iter() {
        //    //let temp = vec![vec![a]];
        //    for a in output2.iter() {
        //        let mut temp2 = a.clone();
        //        temp2.insert(0, begin_word.clone());
        //        output3.push(temp2);
        //    }
        //}
        //
        for (a, begin_word) in output2.iter_mut().zip(begin_world_record.iter()) {
            a.insert(0, begin_word.clone());
        }
    }
    //println!("{:?}",output3);
    //println!("ss");
    (output2, grand_split)
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
            "a".to_string(),
            "c".to_string(),
            vec!["a".to_string(), "b".to_string(), "c".to_string(),]
        )
    );

    println!(
        "{:?}",
        Solution::find_ladders(
            "hot".to_string(),
            "dog".to_string(),
            vec!["hot".to_string(), "dog".to_string(), "dot".to_string(),]
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
                "tm".to_string(),
                "le".to_string(),
                "av".to_string(),
                "sm".to_string(),
                "ar".to_string(),
                "ci".to_string(),
                "ca".to_string(),
                "br".to_string(),
                "ti".to_string(),
                "ba".to_string(),
                "to".to_string(),
                "ra".to_string(),
                "fa".to_string(),
                "yo".to_string(),
                "ow".to_string(),
                "sn".to_string(),
                "ya".to_string(),
                "cr".to_string(),
                "po".to_string(),
                "fe".to_string(),
                "ho".to_string(),
                "ma".to_string(),
                "re".to_string(),
                "or".to_string(),
                "rn".to_string(),
                "au".to_string(),
                "ur".to_string(),
                "rh".to_string(),
                "sr".to_string(),
                "tc".to_string(),
                "lt".to_string(),
                "lo".to_string(),
                "as".to_string(),
                "fr".to_string(),
                "nb".to_string(),
                "yb".to_string(),
                "if".to_string(),
                "pb".to_string(),
                "ge".to_string(),
                "th".to_string(),
                "pm".to_string(),
                "rb".to_string(),
                "sh".to_string(),
                "co".to_string(),
                "ga".to_string(),
                "li".to_string(),
                "ha".to_string(),
                "hz".to_string(),
                "no".to_string(),
                "bi".to_string(),
                "di".to_string(),
                "hi".to_string(),
                "qa".to_string(),
                "pi".to_string(),
                "os".to_string(),
                "uh".to_string(),
                "wm".to_string(),
                "an".to_string(),
                "me".to_string(),
                "mo".to_string(),
                "na".to_string(),
                "la".to_string(),
                "st".to_string(),
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
