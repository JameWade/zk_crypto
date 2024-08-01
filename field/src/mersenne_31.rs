
use std::ops::{Add, Mul, Sub};
use serde::{Deserialize, Serialize};

const P:u32 = (1<<31)-1;
/// The prime field `F_p` where `p = 2^31 - 1`.

#[derive(Copy, Clone, Default, Serialize, Deserialize)]
pub struct Mersenne31{
    pub(crate) value:u32,
}

impl Mersenne31{
    #[inline]
    pub const fn new(value:u32) ->Self{
        Self{
            value: value%P
        }
    }
}
impl Add for Mersenne31{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {

        let (sum_i32, over) = (self.value as i32).overflowing_add(rhs.value as i32);
        let sum_u32 = sum_i32 as u32;
        let sum_corr = sum_u32.wrapping_sub(P);

        Self::new(
            if over { sum_corr }else { sum_u32 }
        )
    }
}

impl Sub for Mersenne31{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        //代码来自plonky3，rust的false bolean值是0
        //然后 Value & ORDER_U32 就是 & p = 2^31 - 1` 等价于 %2^32
        //这里出现溢出是加了 2^32 = 2^31-1+2^31+1
        let (mut sub, over) = self.value.overflowing_sub(rhs.value);

        sub -= over as u32;
        Self::new(sub & P)
    }
}


impl Mul for Mersenne31 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let prod = u64::from(self.value)* u64::from(self.value);
        from_u62(prod)
    }
}
pub(crate) fn from_u62(input:u64) ->Mersenne31{
    debug_assert!(input<(1<<62));
    let input_lo = (input&((1<<31)-1)) as  u32;   //素域以内的留下
    let input_high = (input >>32) as u32;
    Mersenne31::new(input_lo) + Mersenne31::new(input_high)
}

#[cfg(test)]
mod tests{
    #[test]
    fn test(){
        let a= false as u32;
        print!("{}", a);
    }
}