use std::hash::Hash;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

//FiniteField
pub trait Field:
    std::fmt::Debug
    + From<usize>
    + Default
    + Sized
    + Copy
    + Clone
    + PartialEq
    + Eq
    + Add<Output = Self>
    + AddAssign
    + Sum
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Product
    + Div<Output = Self>
    + DivAssign
    + Neg<Output = Self>
    + Rem<Output = Self>
    + Hash
{
    //阶
    const ORDER:usize;
    //加法单位元
    const ZERO:Self;
    //乘法单位元
    const ONE:Self;
    //生成元
    const PRIMITIVE_ELEMENT:Self;
    //计算乘法逆元
    fn inverse(&Self) ->Option<Self>;
    fn pow(self,power:usize)-> Self;
    //计算n次单位根
    fn primitive_root_of_unity(n: usize) -> Self {
        let p_minus_one = Self::ORDER - 1;
        assert_eq!(p_minus_one % n, 0, "n must divide p^q - 1");
        let pow = p_minus_one / n;
        Self::PRIMITIVE_ELEMENT.pow(pow)
    }
}

pub trait PrimeField<const P: usize> {}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
