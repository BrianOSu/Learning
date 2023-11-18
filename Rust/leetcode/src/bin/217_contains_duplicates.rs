use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();

    for i in nums {
        if !set.insert(i){
            return true
        }
    }
    false
}

pub fn contains_duplicate1(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    !nums.into_iter().all(|i| set.insert(i))
}

fn main() {
    println!("1,2,3,1 = {:?}", contains_duplicate(vec!(1,2,3,1)));
    println!("1,2,3,4 = {:?}", contains_duplicate(vec!(1,2,3,4)));
    println!("1,1,1,3,3,4,3,2,4,2 = {:?}", contains_duplicate(vec!(1,1,1,3,3,4,3,2,4,2)));

    println!("1,2,3,1 = {:?}", contains_duplicate1(vec!(1,2,3,1)));
    println!("1,2,3,4 = {:?}", contains_duplicate1(vec!(1,2,3,4)));
    println!("1,1,1,3,3,4,3,2,4,2 = {:?}", contains_duplicate1(vec!(1,1,1,3,3,4,3,2,4,2)));
}
