use std::cmp::Ordering;

pub fn search1(nums: Vec<i32>, target: i32) -> i32 {
    let mut l= 0;
    let mut r= (nums.len()-1) as i32;
    while l<=r {
        let mid = (l + r) / 2;
        if target > nums[mid as usize] {
            l = mid + 1;
        } else if target < nums[mid as usize] {
            r = mid - 1;
        } else {
            return mid;
        }
    }
    -1
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l= 0;
    let mut r= (nums.len()-1) as i32;
    while l<=r {
        let mid = (l + r) / 2;
        match nums[mid as usize].cmp(&target) {
            Ordering::Less => l = mid + 1,
            Ordering::Greater => r = mid - 1,
            Ordering::Equal => return mid
        }
    }
    -1
}

fn main() {
    assert_eq!(search(vec![-1,0,3,5,9,12], 9), 4);
    assert_eq!(search(vec![-1,0,3,5,9,12], 2), -1);
    assert_eq!(search(vec![5], -5), -1);
}