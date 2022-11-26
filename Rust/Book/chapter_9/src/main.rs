mod math;
mod latin;
mod db;

use std::io;
use math::stats;
use std::collections::HashMap;

fn main() {
    let mut v = vec![5,7,6,5,3,4,8,4];
    println!("{:?}", v);
    println!("{}", stats::mean(&v));
    println!("{}", stats::median(&mut v));
    println!("{:?}", stats::mode(&v));
 
    println!("{:?}", latin::pig_latin(&mut String::from("first")));
    println!("{:?}", latin::pig_latin(&mut String::from("apple")));

    // Create the DB to hold the data
    let mut db: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match db::Command::match_input(input){
            Some(db::Command::Add {name, department}) => db::add_to_db(&mut db, department.to_string(), name.to_string()),
            Some(db::Command::All) => db::list_all(&mut db),
            Some(db::Command::List(department)) => db::list_department(&mut db, department.to_string()),
            Some(db::Command::Quit) => break,
            None => println!("Input error!"),
        };

    }
}
