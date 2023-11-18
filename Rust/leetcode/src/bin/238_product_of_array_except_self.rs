pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut right_left: Vec<i32> = vec![1; nums.len()];
    let mut prev_n: i32 = 1;
    let n = nums.len();

    for i in 1..n {
        right_left[i] = nums[i-1] * right_left[i-1];
    }

    for i in 1..n {
        prev_n *= nums[n-i];
        right_left[n-i-1] = right_left[n-i-1] * prev_n;
    }

    right_left
}

fn main() {
    println!("nums = [1,2,3,4] = {:?}", product_except_self(vec![1,2,3,4]));
    println!("nums = [-1,1,0,-3,3] = {:?}", product_except_self(vec![-1,1,0,-3,3]));
}