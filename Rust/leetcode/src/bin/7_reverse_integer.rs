pub fn reverse(x: i32) -> i32 {
    let mut copy = x.clone();
    let mut ans: i32 = 0;
    while copy != 0 {
        if let Some(n) = ans.checked_mul(10){
            ans = n + copy % 10
        } else {
            return 0
        }
        copy /= 10;
    }
    ans
}

fn main() {
    println!("123 {}", reverse(123));
    println!("-123 {}", reverse(-123));
    println!("-23 {}", reverse(-23));
    println!("120 {}", reverse(120));
    println!("1201 {}", reverse(1201));
    println!("1534236469 {}", reverse(1534236469));
    
}
