fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_ending_here = 0;
    let mut max_so_far = i32::MIN;

    for &num in nums {
        max_ending_here = i32::max(num, max_ending_here + num);
        max_so_far = i32::max(max_so_far, max_ending_here);
    }

    max_so_far
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&nums);
    println!("Maximum subarray sum: {}", max_sum);  // Output: 6
}