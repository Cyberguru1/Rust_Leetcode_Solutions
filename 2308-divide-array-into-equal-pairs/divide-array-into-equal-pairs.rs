use std::collections::HashMap;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut b = [u128::MIN; 4];
        for &num in nums.iter() {
          b[(num/128) as usize] ^= (1 << num);
        }
        b.iter().all(|x| x.count_ones() == 0)
    }
}