impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        
        Solution::conv(n)

    }

    pub fn conv(mut n:i32) -> bool{

        if n == 0 {
            return true
        }

        let r = n % 3;

        n = n/3;

        // if r < 0 {
        //     n += 1;
        // }

        return r != 2  && Solution::conv(n)
    }

}