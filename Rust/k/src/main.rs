mod monadic;
mod k;
mod dyadic;

use std::io;
use std::io::{Read, Write};

use k::K;


fn is_global(x: u8) -> bool {
    return x>=b'a' && x<=b'z'
}

fn is_verb(x: u8) -> usize{
    match x {
        b'+' => 1,
        b'-' => 2,
        b'!' => 3,
        b'#' => 4,
        b',' => 5,
        b'@' => 6,
        _ => 0,
    }
}

fn noun(global: &Vec<K>, x: u8) -> K {
    if 47 < x && 58 > x {
        K::Atom(x as i64 - 48)
    } else if is_global(x){
        global[usize::from(x-97)].clone()
    } else {
        K::Error
    }
}

fn eval(global: &Vec<K>, s: &[u8]) -> K{
    if is_verb(s[0]) > 0 {
        let x: K = eval(global, &s[1..]);
        match x {
            K::Error => x,
            _ => monadic::MONADIC[is_verb(s[0])](&x),
        }
    } else {
        let x: K = noun(global, s[0]);
        if K::Error == x  {
            K::Error
        } else if s[1]!=b'\n' && is_verb(s[1])==0 {
            K::Error
        } else if s[1]!=b'\n' {
            let y: K = eval(global, &s[2..]);
            dyadic::DYADIC[is_verb(s[1])](&x, &y)
        } else {
            x
        }
    }
}

fn new_line(){
    print!("k)");
    let _ = io::stdout().flush();
}

fn main() {
    // Global is a vector of pointers to K objects
    let mut global: Vec<K> = Vec::new();
    for _ in 0..26 {
        global.push(K::Error)
    }

    let mut s: [u8; 99] = [0; 99];
    let mut assign: bool;

    new_line();

    while io::stdin().read(&mut s).is_ok(){
        if is_global(s[0]) && s[1] == b':'{
            assign = true;
        } else {
            assign = false;
        }
        let res: K = if assign { eval(&global, &s[2..]) } else { eval(&global, &s) };
        if assign && res!= K::Error {
            println!("{res}");
            println!("{}", (s[0]-97) as usize);
            global[(s[0]-97) as usize] = res;
        } else {
            println!("{res}");
        }
        new_line();
    }
}
