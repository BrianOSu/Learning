use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut mapping: HashMap<i32, i32> = HashMap::new();

    // Create a hashmap for the count of each number
    nums.into_iter().for_each(|num| *mapping.entry(num).or_insert(0) += 1);

    // Convert hashmap to vector to work with
    let mut vec: Vec<(i32, i32)> = mapping.into_iter().collect();

    // Sort the vector by b value greater than a value
    vec.sort_by(|a, b| b.1.cmp(&a.1));

    // Take is used to take the first "k" iterators and map to extract the key
    vec.iter().take(k as usize).map(|x| x.0).collect()
}

fn main() {
    println!("nums = [1,1,1,2,2,1], k = 2 = {:?}", top_k_frequent(vec![1,1,1,2,2,3], 2));
    println!("nums = [1], k = 1 = {:?}", top_k_frequent(vec![1], 1));
}