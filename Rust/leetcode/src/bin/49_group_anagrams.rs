use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut mapping: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    for s in strs{
        let mut key: Vec<char> = s.chars().collect();
        key.sort();
        mapping.entry(key).or_insert(vec![]).push(s)
    }
    mapping.values().cloned().collect()
}

fn main() {
    println!("[\"eat\",\"tea\",\"tan\",\"ate\",\"nat\",\"bat\"]= {:?}", group_anagrams(vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()]));
    println!("[\"\"] = {:?}", group_anagrams(vec!["".to_string()]));
    println!("[\"a\"] = {:?}", group_anagrams(vec!["a".to_string()]));
}