use std::io;

fn far_to_cel() {
    println!("Enter Farenheit to convert to Celcius!");

    let mut farenheit = String::new();

    io::stdin()
        .read_line(&mut farenheit)
        .expect("Failed to read line");

    let farenheit: f32 = farenheit.trim().parse().expect("Please type a number!");

    println!("{}", (farenheit - 32.) * 5. / 9.);

}

fn fib_priv(x: u32) -> u32 {
    if x == 1 || x ==0 {
        x
    } else {
        fib_priv(x-1) + fib_priv(x-2)
    }
}

fn fib(){
    println!("Enter the number of fibonacci terms to compute!");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = number.trim().parse().expect("Please type a number!");
    
    println!("The term is {}", fib_priv(number));
}

fn twelve() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth",
                "seventh", "eight", "ninth", "tenth", "eleventh", "twelfth"];

    let lyrics = ["Twelve drummers drumming",
                    "Eleven pipers piping",
                    "Ten lords a-leaping",
                    "Nine ladies dancing",
                    "Eight maids a-milking",
                    "Seven swans a-swimming",
                    "Six geese a-laying",
                    "Five golden rings",
                    "Four calling birds",
                    "Three French hens",
                    "Two turtle doves",
                    "And a partridge in a pear tree."];
                    
    for day in 0..12 {
        println!("On the {} day of Christmas", days[day]);
        println!("My true love gave to me");
        for i in (0..day+1).rev() {
            println!("{}", lyrics[11-i]);
        }
        println!("");
    }
}

fn main() {
    // Q1 Convert farenheit to celcius
    far_to_cel();

    // Q2 Generate Nth fibonnaci sequence
    fib();
    
    // Q3 Twelve days of Xmas
    twelve();
}
