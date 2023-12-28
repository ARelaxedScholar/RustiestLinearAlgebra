use crate::linear_algebra_number::LinAlgNumber::{Float32, Float64, NaN};
use crate::linear_algebra_number::SafeLinAlgNumber::SafeConversionIsImpossible;
use std::cmp::Ordering::{Less, Greater};

//Class Stuff
const MAX_EXACT_INT: f64 = 9007199254740992.0; //2f64.powi(53);

fn perfectly_representable_as_f64(some_integer: &i64) -> bool{
    !((*some_integer as f64) > MAX_EXACT_INT)   
}

/// Represents the numbers that will be used in the library.
/// Implements the Ord and Eq trait so can be used in data structures that require them.
/// The above is the reason why this enum was created.
/// The NaN variant is handled explicitly to guarantee nothing illegal happened.
/// If a value is stored in a Float variant, then it is a proper number.
#[derive(Clone, Debug)]
pub enum LinAlgNumber {
    Float64(f64),
    Float32(f32),
    NaN,
}

/// Special enum created to handle the From<i64> case.
/// While f64 can store greater values than i64 can, after a certain point (2^53)
/// the f64 type becomes unable to store the integer exactly due to the space required
/// to contain the exponant and the mantissa.
/// Long story short, this is a type made to be used in conjunction with LinAlgNumber.
pub enum SafeLinAlgNumber{
    Safe(LinAlgNumber), //Will only ever be Float64
    SafeConversionIsImpossible
}


impl LinAlgNumber {
    /// A method to check if a float value can be considered an integer.
    /// Its name can be a bit misleading since all it does is check against a threshold
    /// If precision is important, please use the f64 variant.
    /// 
    /// F64 : value * 1*10^(-12)
    /// F32 : value * 1*10^(-6)
    /// NaN : Not a Number (NaN) cannot be an integer
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rustiest_linear_algebra::linear_algebra_number::LinAlgNumber;
    /// 
    /// let var1 : LinAlgNumber = LinAlgNumber::from(64.000001 as f32);
    /// let var2 : LinAlgNumber = LinAlgNumber::from(64.00000001);
    /// assert_eq!(var1.is_basically_an_integer(), true);  threshold : 0.000064 fract_abs : 0.0000001 : fract_abs < threshold => true
    /// assert_eq!(var2.is_basically_an_integer(), false); threshold : 0.000000000064 fract_abs: 00000001  : fract_abs > threshold => false
    /// ```
    pub fn is_basically_an_integer(&self) -> bool {
        match self {
            Float64(value_self) => {
                let relative_epsilon_64bit = value_self.abs() * 1e-12;
                println!("threshold : {}, fractional value: {}", relative_epsilon_64bit, value_self.fract().abs());
                value_self.fract().abs() < relative_epsilon_64bit
            }
            Float32(value_self) => {
                let relative_epsilon_32bit = value_self.abs() * 1e-6;
                println!("threshold : {}, fractional value: {}", relative_epsilon_32bit, value_self.fract().abs());
                value_self.fract().abs() < relative_epsilon_32bit 
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

impl From<i64> for SafeLinAlgNumber {
    fn from(value: i64) -> Self {
        if perfectly_representable_as_f64(&value) {
            SafeLinAlgNumber::from(LinAlgNumber::Float64(value as f64))
        } else {
            SafeConversionIsImpossible
        }
    }
}

impl From<LinAlgNumber> for SafeLinAlgNumber{
    fn from(value: LinAlgNumber) -> Self{
        SafeLinAlgNumber::Safe(value)
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

impl PartialEq<i64> for LinAlgNumber{
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
