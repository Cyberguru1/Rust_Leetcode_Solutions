use std::cmp::max;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {

        if nums.len() == 1 {
            return nums[0];
        }

        let mut pfsum = Vec::new();
        let mut res = 0;

        let mut k = 0;

        nums.iter().for_each(|a| {
            k += a;
            pfsum.push(k);
        });



        let mut st = 0;
        let mut ed = 0;

        for i in 1..nums.len() {
            if nums[i] > nums[i-1] {
                ed += 1;
            }
            
            if nums[i-1] >= nums[i] || i == nums.len() - 1 {
                res = max(res, pfsum[ed] - pfsum[st] + nums[st]);
                st = i;
                ed = i;
            }
        }

        res
    }
}