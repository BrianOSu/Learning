use std::collections::HashMap;

pub fn is_valid1(s: String) -> bool {
    let mut h:HashMap<char,char> = HashMap::new();
    h.insert(')', '(');
    h.insert(']', '[');
    h.insert('}', '{');
    let mut stack = Vec::new();
    for i in s.chars() {
        if ['(', '[', '{'].contains(&i) {
            stack.push(i);
        } else {
            if [')', ']', '}'].contains(&i) {
                if h[&i] == stack.last().unwrap_or_else(|| &' ').clone() {
                    stack.pop();
                } else {
                    return false
                }
            }
        }
    }
    stack.is_empty()
}

pub fn is_valid(s: String) -> bool {
    let mut stack:Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            _ => match stack.pop() {
                Some('(') if c == ')' => (),
                Some('{') if c == '}' => (),
                Some('[') if c == ']' => (),
                _ => return false
            }
        }
    }
    stack.is_empty()
}

fn main() {
    println!("\"()\" = {:?}", is_valid("()".to_string()));
    println!("\"()[]\" = {:?}", is_valid("()[]{}".to_string()));
    println!("\"(]\" = {:?}", is_valid("(]".to_string()));
    println!("\"]\" = {:?}", is_valid("]".to_string()));
}