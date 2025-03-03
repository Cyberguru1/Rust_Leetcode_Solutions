use std::collections::HashMap;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

        let mut map = HashMap::new();
        let mut result: Vec<Vec<i32>> = vec![];

        for con in nums1 {
            map.entry(con[0]).and_modify(|k| {*k += con[1]}).or_insert(con[1]);
        }

        for con in nums2 {
            map.entry(con[0]).and_modify(|k| {*k += con[1]}).or_insert(con[1]);
        }

        for (k, v )in map {
            result.push(vec![k, v]);
        }

        result.sort_by(Solution::compare);


        result
    }

    pub fn compare(a: &Vec<i32>, b: &Vec<i32>) ->std::cmp::Ordering{
    if a[0]>b[0] {
        return std::cmp::Ordering::Greater;
    }
    if a[0]==b[0]{
        return  std::cmp::Ordering::Equal;
    }
    return std::cmp::Ordering::Less;
}
}