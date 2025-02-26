impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {

        let (mut maxe, mut maxsf) = (0, 0);
        let (mut mine, mut minsf) = (0, 0);


        for k in nums {
            maxe = maxe + k;
            mine = mine + k;

            if maxe < 0 {
                maxe = 0;
            }

            if mine > 0 {
                mine = 0;
            }

            maxsf = maxsf.max(maxe);
            minsf = minsf.min(mine);
        }

        maxsf.max(minsf*-1)
    }
}