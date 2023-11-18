use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let h: HashSet<_> = nums.iter().copied().collect();
    let mut ans = 0;
    for &num in &h{
        if !h.contains(&(num - 1)) {
            ans = ans.max((num..).take_while(|x| h.contains(x)).count())
        }
    }
    ans as i32
}

fn main() {
    println!("nums = [100,4,200,1,3,2] = {:?}", longest_consecutive(vec![100,4,200,1,3,2]));
    println!("nums = [0,3,7,2,5,8,4,6,0,1] = {:?}", longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1]));
}