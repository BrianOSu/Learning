use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let user_input = 10;
    let random_input = 7;
    generate_workout(user_input, random_input);
}

fn generate_workout(user_input: u32, random_input: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("Calculating ...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if user_input < 25 {
        println!("Do {} pushups", expensive_closure.value(user_input));
        println!("Do {} situps", expensive_closure.value(random_input));
    } else {
        if random_input ==3 {
            println!("Take a rest day");
        } else {
            println!("Run for {} minutes", expensive_closure.value(user_input));
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}


impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg){
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}