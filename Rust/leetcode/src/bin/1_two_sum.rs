use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut mapping: HashMap<i32, i32> = HashMap::new();

    for (i, x) in nums.iter().enumerate(){
        match mapping.get(x) {
            Some(&n) => return vec!(n, i as i32),
            None => mapping.insert(target - x, i as i32),
        };
    }

    vec![]
}

fn main() {
    println!("2 7 11 15 - 9 = {:?}", two_sum(vec!(2,7,11,15), 9));
    println!("2 11 7 15 - 9 = {:?}", two_sum(vec!(2,11,7,15), 9));
    println!("2 7 11 15 - 26 = {:?}", two_sum(vec!(2,7,11,15), 26));
}
