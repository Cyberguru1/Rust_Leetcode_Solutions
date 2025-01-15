use core::arch::x86_64::_pdep_u32;
use core::cmp::max;

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let p1 = num1.count_ones() as i32;
        let p2 = num2.count_ones() as i32;
        return unsafe { _pdep_u32(u32::MAX << max(p1 - p2, 0), num1 as u32) + _pdep_u32((1 << max(p2 - p1, 0)) - 1, !num1 as u32) } as i32;
    }
}