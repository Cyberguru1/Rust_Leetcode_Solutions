impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {

        if k > blocks.len() as i32 {
            return 0
        }

        let mut cnt = 999999;
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut mcnt = 0;
        let chas: Vec<u8> = blocks.bytes().collect();


        loop {

            if j - i  >= k as usize {
                cnt = cnt.min(mcnt);
                if chas[i] == 87 {
                    mcnt -= 1;
                }
                i += 1;
            }

            if j == chas.len() {
                break
            }

            if chas[j] == 87 {
                mcnt += 1;
            }

            j += 1;

        }

        cnt
        
    }
}