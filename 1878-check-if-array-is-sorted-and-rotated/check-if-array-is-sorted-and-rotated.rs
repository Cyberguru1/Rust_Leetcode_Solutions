impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {

        let mut sorted_array = nums.clone();
        sorted_array.sort();

        // println!("{:?}", sorted_array);


        
        for x in 0..(nums.clone().len() as usize) {
            if sorted_array[(0 + x) % nums.len()] == nums[0] {
                let mut found = true;
                for i in 0..(nums.clone().len()) {
                    if sorted_array[(i + x) % nums.len()] != nums[i] {
                        found = found && false;
                    }
                } 
                if found {
                    return true
                } 
            }else {
                continue
            }
        }

        false
        
    }
}