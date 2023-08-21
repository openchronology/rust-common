#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
#[macro_use]
extern crate lazy_static;

pub mod consts;
pub mod session;

use num_rational::BigRational;
use num_bigint::BigInt;
use quickcheck::{Arbitrary, Gen};
use num_traits::{FromPrimitive, Zero};
use serde::{Serialize, Deserialize, Serializer, Deserializer, de::Error};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct MPQ(pub BigRational);

impl Serialize for MPQ {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        String::serialize(&format!("{}/{}", self.0.numer(), self.0.denom()), serializer)
    }
}

impl<'de> Deserialize<'de> for MPQ {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let s: String = String::deserialize(deserializer)?;
        let mut xs = s.split('/');
        match xs.next() {
            Some(n) => {
                let n = n.parse().map_err(D::Error::custom)?;
                let d = match xs.next() {
                    Some(d) => d.parse().map_err(D::Error::custom),
                    None => Ok(BigInt::from_u8(1).unwrap()),
                }?;
                Result::Ok(MPQ(BigRational::new_raw(n,d)))
            }
            None => Result::Err(D::Error::custom("No numerator")),
        }
    }
}

impl Arbitrary for MPQ {
    fn arbitrary(g: &mut Gen) -> Self {
        let n_to_d = usize::arbitrary(g) % (g.size() - 2);
        let mut gn1 = Gen::new(n_to_d + 1);
        let n1 = BigInt::arbitrary(&mut gn1);
        let mut gd1 = Gen::new((g.size() - n_to_d) + 1);
        let d1 = BigInt::arbitrary(&mut gd1);
        let d1 = if d1.is_zero() {
            BigInt::from_u8(1).unwrap()
        } else {
            d1
        };
        let x = BigRational::new_raw(n1, d1);
        MPQ(x)
    }
}

// SQL identifier type via `serial`
pub type Identifier = u32;

#[cfg(test)]
mod tests {
    #[quickcheck]
    fn mpq_is_serialized_correctly(x: crate::MPQ) -> bool {
        match serde_json::to_string(&x) {
            Err(e) => panic!("Couldn't encode: {:?}", e),
            Ok(y) => match serde_json::from_str::<crate::MPQ>(&y) {
                Err(e) => panic!("Couldn't decode: {:?}", e),
                Ok(z) => x == z,
            }
        }
    }
}
