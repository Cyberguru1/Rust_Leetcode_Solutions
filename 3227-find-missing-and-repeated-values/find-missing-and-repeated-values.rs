use std::collections::HashMap;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {

        let mut hashm = HashMap::new();
        let mut result = vec![0, 0];

        for con in &grid {
            for k in con {
                hashm.entry(*k).and_modify(|a| {result[0] = *k}).or_insert(1);
            }
        }

      

        let r = grid.len() * grid[0].len();

        for v in 1..=r {
            if !hashm.contains_key(&(v as i32)) {
                result[1] = v as i32;
                break
            }
        }
        

        result
        
    }
}