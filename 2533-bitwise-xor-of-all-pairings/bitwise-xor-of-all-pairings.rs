impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {

        let lent1 = nums1.len() as i32;
        let lent2 = nums2.len() as i32;

        let xnum1 = nums1.iter().fold(0, |acc, &x| acc ^ x);
        let xnum2 = nums2.iter().fold(0, |acc, &x| acc ^ x);

        // println!("{lent1}, {lent2}, {xnum1}, {xnum2} {}", xnum1 ^ xnum2);

        if lent1 %2 == 0 && lent2 % 2 == 0 {
            return 0;
        }

        if lent1 % 2 == 0 {
            return xnum1;
        }

        if lent2 % 2 == 0 {
            return xnum2;
        }

        xnum1 ^ xnum2
    }
}