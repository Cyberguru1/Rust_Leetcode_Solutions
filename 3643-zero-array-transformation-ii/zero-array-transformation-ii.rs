impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut prefix_sum = vec![0; nums.len()].into_boxed_slice();
        let mut indices = 0..nums.len();
        let mut ans = 0;
        let mut querries = queries
            .into_iter()
            .map(|query| (query[0] as usize, query[1] as usize, query[2]));

        loop {
            if nums[indices.start] + prefix_sum[indices.start] <= 0 {
                let old_entry = prefix_sum[indices.start];
                indices.next();
                if indices.is_empty() {
                    break;
                }
                prefix_sum[indices.start] += old_entry;
                continue;
            }

            let Some((start, end, val)) = querries.next() else {
                return -1;
            };
            ans += 1;

            if end < indices.start {
                continue;
            }

            if let Some(entry) = prefix_sum.get_mut(end + 1) {
                *entry += val;
            }

            prefix_sum[start.max(indices.start)] -= val;
        }

        ans
    }
}