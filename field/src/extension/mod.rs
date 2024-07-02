use crate::field::Field;

//code from plony3
pub trait BinomiallyExtendable<const D:usize>:Field{
    fn w() ->Self;
    // DTH_ROOT = W^((n - 1)/D).
    // n is the order of base field.
    // Only works when exists k such that n = kD + 1.
    fn dth_root() -> Self;

    fn ext_generator() -> [Self; D];
}