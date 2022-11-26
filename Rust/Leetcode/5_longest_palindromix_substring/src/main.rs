pub fn longest_palindrome(s: String) -> String {
    let l = s.len();
    let s1 = s.clone().chars().rev().collect::<String>();
    println!("{}", s1);

    for i in 0..l+1 {
        println!("{} {}", &s[0..i], &s[i..2*i])
    }

    s
}

pub fn longest_palindrome1(s: String) -> String {
    let l = s.len();
    let mut max = 0;
    let mut max_l = 0;
    let mut max_r =0;

    for i in 0..l{
        for j in (i..l).rev() {
            let n = 1+j-i;
            let mut m = n/2;

            if m%2 != 0 {
                m+=1;
            }

            if &s[i..i+m] == &s[1+j-m..j+1].chars().rev().collect::<String>(){
                if n > max {
                    max_l = i;
                    max_r = 1+j;
                    max = n;
                }
                break;
            }
        }
    }
    (&s[max_l..max_r]).to_string()
}

fn main() {
    println!("palindrome - {}", longest_palindrome(String::from("babad")));
    println!("palindrome - {}", longest_palindrome(String::from("cbbd")));
}
