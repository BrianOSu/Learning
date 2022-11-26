fn main() {
    println!("Hello, world!");

    let mut stack = vec![];
    stack.push(1);
    stack.push(2);
    stack.push(3);

    for i in stack.iter() {
        println!("value is {}", i);
    }

    for (index, i) in stack.iter().enumerate() {
        println!("value is {}, index is {}", i, index);
    }

    while let Some(x) = stack.pop() {
        println!("value is {}", x);
    }

    // vector of tuples
    let nested = vec![("a", "b", "c"), ("d", "e", "f"), ("g", "h", "i")];
    for (a,b,c) in nested.iter(){
        println!("value is {} {} {}", a, b, c);
    }

    for (a,_,c) in nested.iter(){
        println!("value is {} {}", a, c);
    }

    let x = 4;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);


    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

}
