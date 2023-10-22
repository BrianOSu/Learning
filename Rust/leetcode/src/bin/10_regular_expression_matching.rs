pub fn match_dot(s: &str, p: &str) -> bool {
    if s.chars().count() != p.chars().count() {
        return false
    }

    for i in 0..s.chars().count(){
        if s.chars().nth(i).unwrap() != p.chars().nth(i).unwrap() && p.chars().nth(i).unwrap() != '.' {
            return false
        }
    }
    
    true
}

pub fn is_match(s: String, p: String) -> bool {
    if !p.contains('*'){
        return match_dot(&s, &p) 
    }

    //let mut p1 = p.clone();
    //p1.push_str("*");
    
    let mut l = 0;
    let mut r = 0;

    for c in p.split('*') {
        //println!("{}", c);
        r = l + c.chars().count().checked_sub(1).unwrap_or(0);
        if !match_dot(&s[l..r], &c[0..c.len().checked_sub(1).unwrap_or(0)]) {
            return false;
        }
        //while s.chars().nth(r).unwrap() == .chars().nth(r).unwrap() 
        println!("{}", c);
        println!("{} {}", s.chars().nth(r).unwrap(), c.chars().last().unwrap_or(' '));
        while s.chars().nth(r).unwrap() == c.chars().last().unwrap_or(' '){
            println!("{} {}", s.chars().nth(r).unwrap(), c.chars().last().unwrap_or(' '));
            r+=1
        }

        //println!("{} {}", &s[l..r], &s[r..r+1]);
        l=r;
        //println!("{} {}", l, r);
    }
    true
}

// version for actual *
pub fn is_match1(s: String, p: String) -> bool {
    let mut l = 0;
    let mut r = 0;

    for c in p.split('*') {
        r = c.chars().count();
        while !match_dot(&s[l..l+r], c) {
            l+=1;
            if l+r>s.chars().count() {
                return false
            }
        }
        l += r;
    }
    if r != 0 && l!=s.chars().count(){
        return false
    }
    true
}

fn main() {
    //println!("aa a {}", is_match(String::from("aa"), String::from("a")));
    //println!("aa a. {}", is_match(String::from("aa"), String::from("a.")));
    //println!("aa a* {}", is_match(String::from("aa"), String::from("a*")));
    //println!("ab .* {}", is_match(String::from("ab"), String::from(".*")));
    println!("abcdefgh ab.d*h {}", is_match(String::from("abcdefgh"), String::from("ab.d*h")));
    println!("abczefgh ab.d*h {}", is_match(String::from("abczefgh"), String::from("ab.d*h")));
    println!("aab c*a*b {}", is_match(String::from("aab"), String::from("c*a*b")));
}
