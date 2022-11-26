use std::collections::HashMap;

pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 || s.len() < num_rows as usize {
        return s
    }
    let mut n = 0 ;
    let mut pos = false;
    let mut rows: Vec<String> = vec![String::new(); num_rows as usize];

    for (_, c) in s.char_indices() {
        rows[n].push(c.clone());

        if n%(num_rows-1) as usize == 0{
            pos = !pos;
        }
        if pos {
            n += 1;
        } else {
            n -= 1;
        }
    }

    rows.into_iter().collect()
}


pub fn convert1(s: String, num_rows: i32) -> String {
    if num_rows == 1 || s.len() < num_rows as usize {
        return s
    }
    let mut n = 0;
    let mut pos = false;
    let mut level = HashMap::new();

    for (_, c) in s.char_indices() {
        let m = level.entry(n).or_insert("".to_string());
        m.push(c.clone());
        if n%(num_rows-1) == 0{
            pos = !pos;
        }
        if pos {
            n += 1;
        } else {
            n -= 1;
        }
    }

    let mut output = String::new();

    for i in 0..num_rows{
        output.push_str(&level[&i]);
    }

    output
}

fn main() {
    println!("Converted is {}", convert(String::from("PAYPALISHIRING"), 3));
    println!("Converted is {}", convert(String::from("PAYPALISHIRING"), 4));
    println!("Converted is {}", convert(String::from("A"), 1));
}
//PAHNAPLSIIGYIR