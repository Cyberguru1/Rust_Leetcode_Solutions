use std::cmp::max;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {

        let mut inc = vec![1; nums.len()];
        let mut dec = vec![1; nums.len()];
        let (mut maxi, mut maxd) = (1, 1);

        for i in 1..nums.len() {
            if nums[i] > nums[i-1] {
                inc[i] = inc[i-1] + 1;
                maxi = max(maxi, inc[i]);
            }

            if nums[i] < nums[i-1] {
                dec[i] = dec[i-1]+1;
                maxd = max(maxd, dec[i]);
            }
        }
        
        max(maxi, maxd)
    }
}