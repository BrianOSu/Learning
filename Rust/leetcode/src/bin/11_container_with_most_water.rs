use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = height.len() - 1;
    let mut size = 0;

    while l < r {
        size = cmp::max(size , cmp::min(height[l], height[r]) * (r - l) as i32);
        if height[l] <= height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    return size
}


fn main() {
    println!("{}", max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}
