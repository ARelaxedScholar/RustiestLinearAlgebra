use crate::linear_algebra_number::LinAlgNumber::{Float32, Float64, NaN};
use std::cmp::Ordering::{Less, Greater};

//Class Stuff
const MAX_EXACT_INT: f64 = 9007199254740992.0; //2f64.powi(53);

fn perfectly_representable_as_f64(some_integer: &i64) -> bool{
    if (*some_integer as f64) > MAX_EXACT_INT{
        false
    } else {
        true
    }
}


#[derive(Clone, Debug)]
pub enum LinAlgNumber {
    Float64(f64),
    Float32(f32),
    NaN,
}

impl LinAlgNumber {
    fn is_basically_an_integer(&self) -> bool {
        match self {
            Float64(value_self) => {
                let relative_epsilon_64bit = value_self.abs() * 1e-12;
                if value_self.fract().abs() < relative_epsilon_64bit {
                    true
                } else {
                    false
                }
            }
            Float32(value_self) => {
                let relative_epsilon_32bit = value_self.abs() * 1e-6;
                if value_self.fract().abs() < relative_epsilon_32bit {
                    true
                } else {
                    false
                }
            }
            NaN => false,
        }
    }
}

//From Gauntlet
impl From<f64> for LinAlgNumber {
    fn from(value: f64) -> Self {
        if !(value.is_nan()) {
            LinAlgNumber::Float64(value)
        } else {
            NaN
        }
    }
}

impl From<f32> for LinAlgNumber {
    fn from(value: f32) -> Self {
        if !(value.is_nan()) {
            LinAlgNumber::Float32(value)
        } else {
            NaN
        }
    }
}

impl From<i32> for LinAlgNumber {
    fn from(value: i32) -> Self {
        LinAlgNumber::Float64(f64::from(value))
    }
}

impl From<i64> for LinAlgNumber {
    fn from(value: i64) -> Self {
        if !((value as f64) > MAX_EXACT_INT) {
            //Reasonably sure of no precision loss
            LinAlgNumber::Float64(value as f64)
        } else {
            panic!(
                "{} is greater than biggest exact integer representable in f64.",
                value
            )
        }
    }
}
//Impl Comparison traits
impl Eq for LinAlgNumber {}

//Partial Eq Gauntlet
impl PartialEq<LinAlgNumber> for LinAlgNumber {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            //Same type
            (Float64(value_self), Float64(value_other)) => value_self == value_other,
            (Float32(value_self), Float32(value_other)) => value_self == value_other,
            //Different type
            (Float64(value_self), Float32(value_other)) => {
                (*value_self as f64) == (*value_other as f64)
            }
            (Float32(value_self), Float64(value_other)) => {
                (*value_self as f64) == (*value_other as f64)
            }
            //Any NaN
            (NaN, _) | (_, NaN) => false,
        }
    }
}

impl PartialEq<f64> for LinAlgNumber {
    fn eq(&self, other: &f64) -> bool {
        match self {
            Float64(value_self) => *value_self == *other,
            Float32(value_self) => (*value_self as f64) == *other,
            NaN => false,
        }
    }
}

impl PartialEq<f32> for LinAlgNumber {
    fn eq(&self, other: &f32) -> bool {
        match self {
            Float64(value_self) => *value_self == (*other as f64),
            Float32(value_self) => (*value_self as f64) == (*other as f64),
            NaN => false,
        }
    }
}

impl PartialEq<i32> for LinAlgNumber {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Float64(value_self) => {
                if self.is_basically_an_integer() {
                    value_self.trunc() == (f64::from(*other))
                } else {
                    false
                }
            }
            Float32(value_self) => {
                if self.is_basically_an_integer() {
                    (f64::from(value_self.trunc())) == (f64::from(*other))
                } else {
                    false
                }
            }
            NaN => false,
        }
    }
}

impl PartialEq<i64> for LinAlgNumber {
    fn eq(&self, other: &i64) -> bool {
        match self {
            Float64(value_self) => {
                if self.is_basically_an_integer() {
                    if perfectly_representable_as_f64(other) {
                        (f64::from(value_self.trunc())) == (*other as f64)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Float32(value_self) => {
                if perfectly_representable_as_f64(other) {
                    (f64::from(value_self.trunc())) == (*other as f64)
                } else {
                    false
                }
            }
            NaN => false,
        }
    }
}

impl PartialOrd<LinAlgNumber> for LinAlgNumber {
    fn partial_cmp(&self, other: &LinAlgNumber) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Float64(value_self), Float64(value_other)) => value_self.partial_cmp(value_other),
            (Float32(value_self), Float32(value_other)) => {
                (f64::from(*value_self)).partial_cmp(&f64::from(*value_other))
            }
            (Float64(value_self), Float32(value_other)) => {
                value_self.partial_cmp(&f64::from(*value_other))
            }
            (Float32(value_self), Float64(value_other)) => {
                (f64::from(*value_self)).partial_cmp(value_other)
            }
            (NaN, _) => None,
            (_, NaN) => None,
        }
    }
}

//TODO: PartialOrd! Also rework functions since just realized at no point I check that
// the f64/f32 are not NaN.
impl PartialOrd<f64> for LinAlgNumber {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        match self {
            Float64(value_self) => value_self.partial_cmp(other),
            Float32(value_self) => (f64::from(*value_self)).partial_cmp(other),
            NaN => None,
        }
    }
}

impl PartialOrd<f32> for LinAlgNumber {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        match self {
            Float64(value_self) => value_self.partial_cmp(&f64::from(*other)),
            Float32(value_self) => (f64::from(*value_self)).partial_cmp(&f64::from(*other)),
            NaN => None,
        }
    }
}

impl PartialOrd<i32> for LinAlgNumber {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        match self {
            Float64(value_self) => value_self.partial_cmp(&f64::from(*other)),
            Float32(value_self) => (f64::from(*value_self)).partial_cmp(&f64::from(*other)),
            NaN => None,
        }
    }
}

impl PartialOrd<i64> for LinAlgNumber {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        match self {
            Float64(value_self) => value_self.partial_cmp(&(*other as f64)),
            Float32(value_self) => (f64::from(*value_self)).partial_cmp(&(*other as f64)),
            NaN => None,
        }
    }
}

//ORD!
impl Ord for LinAlgNumber{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other){
            //Since we ensure that Float64/Float32 do not contain NaN.
            //Comparison between these types should never fail. So we unwrap for brevity
            (NaN, _) => Greater,
            (_, NaN) => Less,
            (Float64(_), Float64(_)) => self.partial_cmp(other).unwrap(),
            (Float64(_), Float32(_)) => self.partial_cmp(other).unwrap(),
            (Float32(_), Float64(_)) => self.partial_cmp(other).unwrap(),
            (Float32(_), Float32(_)) => self.partial_cmp(other).unwrap(),
            
        }
    }
}
