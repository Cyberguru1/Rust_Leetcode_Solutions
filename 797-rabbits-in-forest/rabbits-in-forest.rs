use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut mappings = HashMap::<i32, i32>::new();

        let mut con = answers;
        con.sort();
        let mut res = 0;


        for k in con {
            if mappings.contains_key(&k) {
                let value = mappings.get(&k);
                if *value.unwrap() == k + 1 {
                    mappings.insert(k, 1);
                    res += k + 1;
                    continue
                }
                mappings.entry(k).and_modify(|j| *j += 1);
            }else {
                mappings.insert(k, 1);
                res += k + 1;
            }

        }
        res
    }
}