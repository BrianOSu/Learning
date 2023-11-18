use std::collections::HashMap;
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len(){
        return false
    }

    let mut mapping: HashMap<char, i32> = HashMap::new();
    s.chars().for_each(|c| *mapping.entry(c).or_insert(0)+=1);
    t.chars().for_each(|c| *mapping.entry(c).or_insert(0)-=1);
    !mapping.into_values().any(|val| val!=0)
}

fn main() {
    println!("anagram, nagaram = {:?}", is_anagram("anagram".to_string(), "nagaram".to_string()));
    println!("rat, car = {:?}", is_anagram("rat".to_string(), "car".to_string()));
}
