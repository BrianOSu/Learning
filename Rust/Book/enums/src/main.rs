#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColour
}

impl Message {
    fn call(&self) {
        println!("The messafe is this {:?}", self);
    }
}


#[derive(Debug)]
enum UsStates{
    Alaska,
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin{
        Coin::Penny => 1,
        Coin::Dime => 5,
        Coin::Nickel => 10,
        Coin::Quarter(state) =>{
            println!("hey, it's an {:?}", state);
            25
        }
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("Hello, world! {:?} {:?}", home, loopback);

    let m = Message::Write(String::from("test"));
    m.call();

    let my_coin = Coin::Quarter(UsStates::Alabama);
    println!("The coin value is {}", value_in_cents(my_coin));

}
