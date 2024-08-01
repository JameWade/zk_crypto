use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt::{Debug, Display};

// Abstract Field ==> field ==> PrimeField
pub trait AbstractField:
Sized + Clone + Debug +
Add<Output = Self> + Sub<Output = Self> +
Mul<Output = Self> + Neg<Output = Self>
{
    type F: Field;

    fn zero() -> Self;
    fn one() -> Self;
    fn from_u64(n: u64) -> Self;
    fn generator() -> Self;
    fn neg_one() -> Self;

    fn square(&self) -> Self {
        self.clone() * self.clone()
    }

    fn pow(&self, exp: u64) -> Self {
        let mut base = self.clone();
        let mut result = Self::one();
        let mut exp = exp;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base.clone();
            }
            base = base.square();
            exp >>= 1;
        }
        result
    }
}

// Field trait
pub trait Field: AbstractField<F = Self> + Copy + Eq + Display {
    fn inverse(&self) -> Option<Self>;
    fn order() -> u64;
}

// PrimeField trait
pub trait PrimeField: Field {
    fn as_u64(&self) -> u64;
}

// Concrete implementation of a prime field
const P:u32 = 17;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimeFieldElement {
    value: u64,
    prime: u64,
}

impl PrimeFieldElement {
    pub fn new(value: u64, prime: u64) -> Self {
        PrimeFieldElement { value: value % prime, prime }
    }
}

impl AbstractField for PrimeFieldElement {
    type F = Self;

    fn zero() -> Self {
        PrimeFieldElement::new(0, Self::order())
    }

    fn one() -> Self {
        PrimeFieldElement::new(1, Self::order())
    }

    fn from_u64(n: u64) -> Self {
        PrimeFieldElement::new(n, Self::order())
    }


    fn generator() -> Self {
        PrimeFieldElement::new(2, Self::order()) // Simplified; not always correct
    }

    fn neg_one() -> Self {
        Self::new(Self::order() - 1, Self::order())
    }
}

impl Field for PrimeFieldElement {
    fn inverse(&self) -> Option<Self> {
        if self.value == 0 {
            None
        } else {
            Some(self.pow(self.prime - 2))
        }
    }

    fn order() -> u64 {
        P.into()
    }
}

impl PrimeField for PrimeFieldElement {
    fn as_u64(&self) -> u64 {
        self.value
    }
}

impl Add for PrimeFieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        PrimeFieldElement::new(self.value + other.value, self.prime)
    }
}

impl Sub for PrimeFieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        PrimeFieldElement::new(self.prime + self.value - other.value, self.prime)
    }
}

impl Mul for PrimeFieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        PrimeFieldElement::new(self.value * other.value, self.prime)
    }
}

impl Neg for PrimeFieldElement {
    type Output = Self;

    fn neg(self) -> Self {
        PrimeFieldElement::new(self.prime - self.value, self.prime)
    }
}

impl Div for PrimeFieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inverse().expect("Division by zero")
    }
}

impl Display for PrimeFieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// Example usage
#[test]
fn test_field() {
    let a = PrimeFieldElement::new(3, 17);
    let b = PrimeFieldElement::new(8, 17);
    println!("a + b = {}", a + b);
    println!("a * b = {}", a * b);
    println!("a^3 = {}", a.pow(3));
    println!("1/a = {:?}", a.inverse());
}