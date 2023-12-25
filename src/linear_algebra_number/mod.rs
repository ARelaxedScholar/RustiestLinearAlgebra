
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord)]
pub enum LinAlgNumber{
    Float64(f64),
    Float32(f32),
    NaN,
    None
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
//TODO: WRITE THE COMPARISON LOGIC