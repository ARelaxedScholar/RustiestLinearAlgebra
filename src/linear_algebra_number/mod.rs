use crate::linear_algebra_number::LinAlgNumber::{Float64, Float32, NaN};

#[derive(Copy, Clone, Debug, PartialOrd, Ord)]
pub enum LinAlgNumber{
    Float64(f64),
    Float32(f32),
    NaN,
}
//From Gauntlet
impl From<f64> for LinAlgNumber{
    fn from(value : f64) -> Self{
        LinAlgNumber::Float64(value)
    }
}

impl From<f32> for LinAlgNumber{
    fn from(value : f32) -> Self{
        LinAlgNumber::Float32(value)
    }
}

impl From<i32> for LinAlgNumber{
    fn from(value : i32) -> Self{
        LinAlgNumber::Float64(f64::from(value))
    }
}
//Impl Comparison traits
impl Eq for LinAlgNumber{}
//Partial Eqs
impl PartialEq<LinAlgNumber> for LinAlgNumber{ 
    fn eq(&self, other: &Self) -> bool {
       match (self, other) {
        //Same type
        (Float64(value_self), Float64(value_other)) => value_self == value_other,
        (Float32(value_self), Float32(value_other)) => value_self == value_other,
        //Different type
        (Float64(value_self), Float32(value_other)) => (*value_self as f64) == (*value_other as f64),
        (Float32(value_self), Float64(value_other)) => (*value_self as f64) == (*value_other as f64),
        //Any NaN
        (NaN, _) | (_, NaN) => false
       }
}}

impl PartialEq<f64> for LinAlgNumber{
    fn eq(&self, other: &f64) -> bool {
        match self{
            Float64(value_self) => *value_self == *other,
            Float32(value_self) => (*value_self as f64) == *other,
            NaN => false
        }
    }
}

impl PartialEq<f32> for LinAlgNumber{
    fn eq(&self, other: &f32) -> bool {
        match self{
            Float64(value_self) => *value_self == (*other as f64),
            Float32(value_self) => (*value_self as f64) == (*other as f64),
            NaN => false
        }
    }
}

//TODO: Write comparison logic agains i32 and i64 for PartialEq, and then define the PartialOrd







