use crate::linear_algebra_number::LinAlgNumber::{Float64, Float32, NaN};

const MAX_EXACT_INT: f64 = 2f64.powi(53);

#[derive(Clone, Debug)]
pub enum LinAlgNumber{
    Float64(f64),
    Float32(f32),
    NaN
}


//From Gauntlet
impl From<f64> for LinAlgNumber{
    fn from(value : f64) -> Self{
        if !(value.is_nan()){
            LinAlgNumber::Float64(value)
        } else {
            NaN
        }  
    }
}

impl From<f32> for LinAlgNumber{
    fn from(value : f32) -> Self{
        if !(value.is_nan()){
            LinAlgNumber::Float32(value)
        } else {
            NaN
        }  
    }
}

impl From<i32> for LinAlgNumber{
    fn from(value : i32) -> Self{
        LinAlgNumber::Float64(f64::from(value))
    }
}

impl From<i64> for LinAlgNumber{
    fn from(value: i64) -> Self {
        if !((value as f64) > MAX_EXACT_INT) {
            //Reasonably sure of no precision loss
            LinAlgNumber::Float64(value as f64)
        }
        else {
           panic!("{} is greater than biggest exact integer representable in f64.", value)
        }
        
    }
}
//Impl Comparison traits
impl Eq for LinAlgNumber{}

//Partial Eq Gauntlet
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

impl PartialEq<i32> for LinAlgNumber{
    fn eq(&self, other:&i32) -> bool {
        match self {
            Float64(value_self) => {
                let relative_epsilon_64bit = value_self.abs() * 1e-12;
                if value_self.fract().abs() < relative_epsilon_64bit{
                    //Essentially an integer, we can try to compare
                    value_self.trunc() == (f64::from(*other))
                } else {
                    false
                }
            },
            Float32(value_self) => {
                let relative_epsilon_32bit = value_self.abs() * 1e-6;
                if value_self.fract().abs() < relative_epsilon_32bit {
                    (f64::from(value_self.trunc())) == (f64::from(*other))
                } else{
                    false
                }

            },
            NaN => false
        }


    }
}

impl PartialEq<i64> for LinAlgNumber{
    fn eq(&self, other:&i64) -> bool{
        match self{
            Float64(value_self) => {
                let relative_epsilon_64bit = value_self * 1e-12;
                if value_self.fract().abs() < relative_epsilon_64bit {
                    if !(&(*other as f64) > &MAX_EXACT_INT){
                        (f64::from(value_self.trunc())) == (*other as f64)
                    } else {
                        false
                    }
                    
                } else {
                    false
                }

            },
            Float32(value_self) => {
                let relative_epsilon_32bit = value_self.abs() * 1e-6;
                if value_self.fract().abs() < relative_epsilon_32bit{
                    (f64::from(value_self.trunc())) == (f64::from(*other))
                } else {
                    false
                }

            },
            NaN => false
        }

    }
}


impl PartialOrd<LinAlgNumber> for LinAlgNumber{
    fn partial_cmp(&self, other: &LinAlgNumber) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Float64(value_self), Float64(value_other)) => value_self.partial_cmp(value_other),
            (Float32(value_self), Float32(value_other)) => (f64::from(*value_self)).partial_cmp(&f64::from(*value_other)),
            (Float64(value_self), Float32(value_other)) => value_self.partial_cmp(&f64::from(*value_other)),
            (Float32(value_self), Float64(value_other)) => (f64::from(*value_self)).partial_cmp(value_other),
            (NaN, _) => None,
            (_, NaN) => None,
        }
    }
}

//TODO: PartialOrd! Also rework functions since just realized at no point I check that
// the f64/f32 are not NaN.
impl PartialOrd<f64> for LinAlgNumber{
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl PartialOrd<f32> for LinAlgNumber{
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl PartialOrd<i32> for LinAlgNumber{
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl PartialOrd<i64> for LinAlgNumber{
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        todo!()
    }
}







