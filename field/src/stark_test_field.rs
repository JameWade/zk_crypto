use std::fmt::{Debug, Formatter};
use std::ops::{Add, Mul, Neg, Sub};
use crate::field::AbstractField;
use ff::{BitViewSized, Field as FFField, PrimeField as FFPrimeField, PrimeFieldBits};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize};

const P:u128 = 1 + 407 * ( 1 << 119 );
#[derive(FFPrimeField)]
#[PrimeFieldModulus = "270497897142230380135924736767050121217"]
#[PrimeFieldReprEndianness = "little"]
pub struct FFTestFr([u64;4]);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestField{
    value:FFTestFr,
}
impl Serialize for TestField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let repr = self.value.to_repr();
        let bytes = repr.as_ref();

        let mut seq = serializer.serialize_seq(Some(bytes.len()))?;
        for e in bytes {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for TestField {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes: Vec<u8> = Deserialize::deserialize(d)?;

        let mut res = <TestField as FFPrimeField>::Repr::default();

        for (i, digit) in res.0.as_mut().iter_mut().enumerate() {
            *digit = bytes[i];
        }

        let value = TestField::from_repr(res);

        if value.is_some().into() {
            Ok(Self {
                value: value.unwrap(),
            })
        } else {
            Err(serde::de::Error::custom("Invalid field element"))
        }
    }
}

impl Add<Output=Self> for TestField {
    type Output = ();

    fn add(self, rhs: Self) -> Self::Output {
      Self::new(self.value+rhs.value)
    }
}

impl Sub<Output=Self> for TestField {
    type Output = ();

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value-rhs.value)
    }
}

impl Mul<Output=Self> for TestField {
    type Output = ();

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value*rhs.value)
    }
}

impl Neg<Output=Self> for TestField {
    type Output = ();

    fn neg(self) -> Self::Output {
       self*Self::neg_one()
    }
}

impl AbstractField for TestField {
    type F = ();

    fn zero() -> Self {
        Self::new(FFTestFr::ZERO)
    }

    fn one() -> Self {
        Self::new(FFTestFr::ONE)
    }

    fn from_u64(n: u64) -> Self {
        todo!()
    }

    fn generator() -> Self {
        todo!()
    }

    fn neg_one() -> Self {
        todo!()
    }
}
#[test]
fn test_num(){
    print!("{:?}",P)
}