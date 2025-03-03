impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        
        let mut lessarr = vec![];
        let mut greatarr = vec![];
        let mut result = vec![];

        for k in nums {
            if k < pivot {
                lessarr.push(k);
            }else if k > pivot {
                greatarr.push(k);
            }else {
                result.push(k);
            }
        }

        result.extend_from_slice(&greatarr);
       [lessarr, result].concat()
    }
}