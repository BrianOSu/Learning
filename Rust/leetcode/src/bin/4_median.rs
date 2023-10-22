pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums = vec![];
    let mut i = 0;
    let mut j = 0;

    while i<nums1.len() || j<nums2.len(){
        if i==nums1.len() {
            nums.extend_from_slice(&nums2[j..nums2.len()]);
            j=nums2.len();
        } else if j==nums2.len() {
            nums.extend_from_slice(&nums1[i..nums1.len()]); 
            i=nums1.len();
        } else if nums1[i] < nums2[j] {
            nums.push(nums1[i]);
            i += 1;
        } else {
            nums.push(nums2[j]);
            j += 1;
        }
    }
    
    let mid = nums.len() / 2;
    if nums.len() % 2 == 0 {
        (nums[mid-1] + nums[mid]) as f64 / 2.0
    } else {
        nums[mid] as f64
    }
}

fn main() {
    println!("median {}", find_median_sorted_arrays(vec![1,3], vec![2]));
    println!("median {}", find_median_sorted_arrays(vec![1,2], vec![3,4]));
}
