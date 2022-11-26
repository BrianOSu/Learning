use std::collections::HashMap;
use std::cmp;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max = 0;
    let len = s.len();

    for j in 0..len{
        let mut m = HashMap::new();
        for (i, c) in s[j..len].char_indices(){
            match m.get(&c){
                Some(_) => break,
                None => {
                            m.insert(c, i);
                            max = cmp::max(max, i+1);
                        },
            }
        }
    }
    max as i32
}

fn main() {
    println!("{}", length_of_longest_substring(String::from("abcabcbb")));
    println!("{}", length_of_longest_substring(String::from(" ")));
    println!("{}", length_of_longest_substring(String::from("au")));
}