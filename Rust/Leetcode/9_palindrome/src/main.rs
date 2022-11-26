pub fn one_num(x: i32) -> bool{
    match x {
        0..=9 => true,
        _ => false
    }
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    if one_num(x) {
        return true
    }

    let mut copy = x;
    let mut rev = 0;
    while copy > 0 {
        rev *= 10;
        rev += copy % 10;
        copy /= 10;
    }

    rev == x
}



fn main() {
    println!("23 {}", is_palindrome(23));
    println!("121 {}", is_palindrome(121));
    println!("0 {}", is_palindrome(0));
    println!("-10 {}", is_palindrome(-10));
}
