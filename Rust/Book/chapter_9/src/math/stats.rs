use std::collections::HashMap;

pub fn mean(x: &[i32])-> f32{
    x.iter().sum::<i32>() as f32 / x.len() as f32
}

pub fn median(x: &mut [i32])-> i32{
    x.sort();
    x[x.len()/2]
}

pub fn mode(x: &[i32]) -> Vec<i32>{
    let mut h = HashMap::new();
    for i in x.iter(){
        let count = h.entry(i).or_insert(0);
        *count += 1;
    }

    let max_value = h.values().cloned().max().unwrap_or(0);
    
    h.into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(&k, _)| k)
        .collect()
}