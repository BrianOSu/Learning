pub fn my_atoi(s: String) -> i32 {
    let mut x:i32 = 0;
    let copy = s.trim().clone();
    
    let (copy,sign) = match copy.get(..1) {
                    Some("-") => (&copy[1..copy.len()], -1),
                    Some("+") => (&copy[1..copy.len()], 1),
                    _ => (copy, 1),
               };

    for (i,c) in copy.char_indices(){
        if 47 < c as i32 && 58 > c as i32 {
            match x.checked_mul(10){
                Some(n) => x=n,
                None => return if sign > 0 {i32::MAX} else {i32::MIN},
            }

            match x.checked_add(-48 + c as i32){
                Some(n) => x=n,
                None => return if sign > 0 {i32::MAX} else {i32::MIN},
            }
        }
        else {
            return x*sign
        }
    }
    x*sign
}

fn main() {
    println!("123 {}",my_atoi(String::from(" 123")));
    println!("+123 {}",my_atoi(String::from(" +123")));
    println!("-123 {}",my_atoi(String::from(" -123")));
    println!("-123 test {}",my_atoi(String::from(" -123 test")));
    println!("-91283472332 {}",my_atoi(String::from("-91283472332")));
    println!("21474836460 {}",my_atoi(String::from("21474836460")));
}
