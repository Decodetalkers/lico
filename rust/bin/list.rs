#[derive(Clone)]
enum List {
    Cons(String, Box<List>),
    Nil,
}
use List::*;
impl List {
    fn new() -> Self {
        Nil
    }

    fn prepend(self, elem: String) -> Self {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(ref head, ref tail) => {
                format!("{},{}", head.clone(), tail.stringify())
            }
            Nil => "Nil".to_string(),
        }
    }
    fn to_vec(&self) -> Vec<String> {
        let mut output = vec![];
        if let Cons(ref head, ref tail) = *self {
            let mut temp = tail.to_vec();
            output.append(&mut temp);
            output.push(head.to_string());
        }
        output
    }
}
type ListVec = Vec<List>;
trait IntoVec {
    fn as_vec(&self) -> Vec<Vec<String>>;
}
impl IntoVec for ListVec {
    fn as_vec(&self) -> Vec<Vec<String>> {
        let mut output = vec![];
        for avec in self {
            output.push(avec.to_vec())
        }
        output
    }
}
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
        let mut list = List::new();
        list = list.prepend(begin_word);
        find_ladders_before(vec![list], end_word, before).as_vec()
    }
}
fn find_ladders_before(
    begin_words: Vec<List>,
    end_word: String,
    word_list: Vec<String>,
) -> Vec<List> {
    if word_list.is_empty() {
        return vec![];
    }
    let mut word_list2 = word_list.clone();

    let mut stop_vec = vec![];
    let mut next_vec = vec![];
    let mut to_remove = vec![];
    for begin_word in begin_words {
        if let Cons(ref head, _) = begin_word {
            for (i, astring) in word_list.iter().enumerate() {
                let mut count = 0;
                let mut is_same = true;
                for (a, (b, c)) in head
                    .clone()
                    .chars()
                    .zip(astring.chars().zip(end_word.clone().chars()))
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
                        let mut list = begin_word.clone();
                        list = list.prepend(end_word.clone());
                        stop_vec.push(list);
                    } else {
                        let mut list = begin_word.clone();
                        list = list.prepend(astring.to_string());
                        next_vec.push(list);
                    }
                    to_remove.push(i);
                }
            }
        }
    }
    to_remove.sort_unstable();
    to_remove.reverse();
    let mut to_remove2 = vec![];
    let mut temp = -1;
    for i in to_remove {
        if temp != i as i32 {
            to_remove2.push(i);
            temp = i as i32;
        }
    }
    for i in to_remove2 {
        word_list2.remove(i);
    }
    if !stop_vec.is_empty() {
        stop_vec
    } else if next_vec.is_empty() {
        vec![]
    } else {
        find_ladders_before(next_vec, end_word, word_list2)
    }
    //vec![]
}
fn main() {
    let mut list = List::new();
    list = list.prepend(1.to_string());
    list = list.prepend(2.to_string());
    list = list.prepend(3.to_string());

    println!("linked list has length:{}", list.len());
    println!("{}", list.stringify());
    println!("{:?}", list.to_vec());
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
            ]
        )
    );
    println!(
        "{:?}",
        Solution::find_ladders(
            "hot".to_string(),
            "dog".to_string(),
            vec!["hot".to_string(), "dog".to_string(),]
        )
    );
}
