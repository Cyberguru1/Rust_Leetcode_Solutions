use std::collections::HashMap;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {

        let lent = a.len();
        let mut hashmap = HashMap::<i32, u8>::new();
        let mut hashmapch = Vec::new();
        let mut res: Vec<i32> = Vec::new();

        for i in 0..lent {
            hashmap
                .entry(a[i])
                .and_modify(|d| {
                    hashmapch.push(a[i]);
                    *d += 1;
                })
                .or_insert(1);

            hashmap
                .entry(b[i])
                .and_modify(|d| {
                    hashmapch.push(b[i]);
                    *d += 1;
                })
                .or_insert(1);
            
            res.push(hashmapch.len() as i32);
        }
        res 
    }
}