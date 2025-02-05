use std::collections::HashMap;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {

        if s1 == s2 {
            return true
        }


        if s1.len() != s2.len() {
            return false
        }

        let mut hmaps1 = HashMap::new();
        let mut hmaps2 = HashMap::new();

        for i in 0..s1.len() {

            hmaps1.entry(s1.as_bytes()[i])
                .and_modify(|a| *a += 1)
                .or_insert(1);
            
            hmaps2.entry(s2.as_bytes()[i])
                .and_modify(|a| *a += 1)
                .or_insert(1);
        }


        let mut sm: usize = 0;

        for i in 0..s1.len() {
            
            let (k1, k2) = (hmaps1.get(&s1.as_bytes()[i]), hmaps2.get(&s1.as_bytes()[i]));

            if k1 != k2 {
                return false
            }
            
            if s1.as_bytes()[i] == s2.as_bytes()[i] {
                sm += 1
            }
        }

        if sm == s1.len() - 2 {
            return true
        }


        false
    }
}