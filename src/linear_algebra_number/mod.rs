use crate::linear_algebra_number::LinAlgNumber::{Float32, Float64, NaN};
use crate::linear_algebra_number::SafeLinAlgNumber::SafeConversionIsImpossible;
use std::cmp::Ordering::{Less, Greater};

// For my own sanity, and because the point of this is to do machine learning at some point
// and not just to labor in rust traits implementation. I will simply implement a to_inner method
// and make use of the inherent arithmetic capabilities of floats.
// I will learn about macros later.
// TODO: Write documentation for the comparison logic, write the inner.

//Class Stuff
const MAX_EXACT_INT: i64 = 9007199254740991; //2f64.powi(53)-1;

fn perfectly_representable_as_f64(some_integer: &i64) -> bool{
    !(((*some_integer).abs()) > MAX_EXACT_INT)   // The max is also the minimum, since f64 in IEEE754 is signed magnitude.
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SafeLinAlgNumber{
    Safe(LinAlgNumber), //Will only ever be Float64
    SafeConversionIsImpossible(String)
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
    /// let var1 : LinAlgNumber = LinAlgNumber::Float32(64.000001 as f32);
    /// let var2 : LinAlgNumber = LinAlgNumber::Float64(64.00000001);
    /// assert_eq!(var1.is_basically_an_integer(), true);  //threshold : 0.000064 fract_abs : 0.0000001 : fract_abs < threshold => true
    /// assert_eq!(var2.is_basically_an_integer(), false); //threshold : 0.000000000064 fract_abs: 00000001  : fract_abs > threshold => false
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

    /// A method to check if the stored variant is a NaN.
    /// It is needed since NaN != NaN. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rustiest_linear_algebra::linear_algebra_number::LinAlgNumber;
    /// use rustiest_linear_algebra::linear_algebra_number::LinAlgNumber::{Float64, Float32, NaN};
    /// 
    /// let var1 : LinAlgNumber = Float64(5.0);
    /// let var2 : LinAlgNumber = Float32(1.11);
    /// let var3 : LinAlgNumber = NaN;
    /// 
    /// assert_eq!(var1.is_nan(), false);
    /// assert_eq!(var2.is_nan(), false);
    /// assert_eq!(var3.is_nan(), true);
    /// 
    /// ```

    pub fn is_nan(&self) -> bool {
        match self{
            NaN => true,
            _ => false
        }
    }
}

//From Gauntlet
/// A method to transform f64 into the wrapper type LinAlgNumber
/// It will always yield a Float64 variant (which is the favored option.)
/// Since we use the variant NaN to represent Not A Number values there's a guarantee that
/// once a float enters the system, if it was valid during entry it will remain valid while in.
/// 
/// # Examples
/// 
/// ```
/// use rustiest_linear_algebra::linear_algebra_number::LinAlgNumber;
/// use rustiest_linear_algebra::linear_algebra_number::LinAlgNumber::{Float64, NaN};
/// 
/// let var1 : LinAlgNumber = LinAlgNumber::from(7.77);
/// let var2 : LinAlgNumber = LinAlgNumber::from(4e-78);
/// let var3 : LinAlgNumber = LinAlgNumber::from(7e100);
/// let var4 : LinAlgNumber = LinAlgNumber::from((f64::from(-1)).sqrt()); 
/// let var5 : LinAlgNumber = LinAlgNumber::from(0.0/0.0);
/// 
/// assert_eq!(var1, Float64(7.77));
/// assert_eq!(var2, Float64(4e-78));
/// assert_eq!(var3, Float64(7e100));
/// assert_eq!(var1.is_nan(), false);
/// assert_eq!(var2.is_nan(), false);
/// assert_eq!(var3.is_nan(), false);
/// assert_eq!(var4.is_nan(), true);
/// assert_eq!(var5.is_nan(), true);
/// ```
impl From<f64> for LinAlgNumber {
    fn from(value: f64) -> Self {
        if !(value.is_nan()) {
            LinAlgNumber::Float64(value)
        } else {
            NaN
        }
    }
}


/// A method to transfomr f32 into the wrapper type.
/// Implemented for completeness but I wouldn't recommend using it for precision reasons.
/// The expected behavior is identical to Float64. 
/// This is the from that is used when one explicitly passes a F32 to the from method.
/// 
/// # Examples
/// 
/// ```
/// use rustiest_linear_algebra::linear_algebra_number::LinAlgNumber;
/// 
/// let var1 : LinAlgNumber = LinAlgNumber::from(34.0); // passing a regular f64
/// let var2 : LinAlgNumber = LinAlgNumber::from(34.0 as f32);
/// let var3 : LinAlgNumber = LinAlgNumber::from(f32::from(-34.0).sqrt()); //Taking square root of negative.
/// 
/// assert_eq!(var1, LinAlgNumber::Float64(34.0));
/// assert_eq!(var2, LinAlgNumber::Float32(34.0));
/// assert_eq!(var1.is_nan(), false);
/// assert_eq!(var2.is_nan(), false);
/// assert_eq!(var3.is_nan(), true);
/// ``` 
impl From<f32> for LinAlgNumber {
    fn from(value: f32) -> Self {
        if !(value.is_nan()) {
            LinAlgNumber::Float32(value)
        } else {
            NaN
        }
    }
}

/// This is a method to transform an i32 into the wrapper type Float64 (always Float64).
/// This method is here as Quality of Life feature, since all the comparisons and operations are done using float numbers.
/// This is merely there so that a user can create new LinAlgNumber instances without having to write the number as a float literal.
/// 
/// For i32 doing so is as simple as casting the number to f64 and storing that in the enum.
/// This is possible since the range of values of i32 is much smaller than the range representable exactly by a f64.
/// 
/// # Examples
/// ```
/// use rustiest_linear_algebra::linear_algebra_number::LinAlgNumber;
/// 
/// let var1 : LinAlgNumber = LinAlgNumber::from(459877902); //in rust, by default, integers are i32 if not otherwise specified. 
/// let var2 : LinAlgNumber = LinAlgNumber::from(0);
/// let var3 : LinAlgNumber = LinAlgNumber::from(-459877902);
/// 
/// assert_eq!(var1, LinAlgNumber::Float64(459877902.0));
/// assert_eq!(var2, LinAlgNumber::Float64(0.0));
/// assert_eq!(var3, LinAlgNumber::Float64(-459877902.0));
/// 
/// ```
impl From<i32> for LinAlgNumber {
    fn from(value: i32) -> Self {
        LinAlgNumber::Float64(f64::from(value))
    }
}

/// This is a method to transform an i64 into the wrapper type.
/// Since as opposed to i32, i64 **can** exhaust the range of **exactly** representable numbers in f64
/// and because I am not brave enough to implement method for f128. 
/// There is a risk of precision loss each time we convert such a number.
/// 
/// As a result, we introduce a new enum SafeLinAlgNumber which serves as a homebrewed-highly-specified Result enum.
/// It yields either:
/// - a Safe variant containing a Float64 variant of LinAlgNumber,
/// - SafeConversionImpossible variant for when the number is too big to be represented exactly.
/// 
/// This offers the user some discretion with how they intend to deal with failure of conversion.
/// 
/// # Examples
/// ```
/// use rustiest_linear_algebra::linear_algebra_number::LinAlgNumber::Float64;
/// use rustiest_linear_algebra::linear_algebra_number::SafeLinAlgNumber;
/// use rustiest_linear_algebra::linear_algebra_number::SafeLinAlgNumber::{Safe, SafeConversionIsImpossible};
/// 
/// let var1 : SafeLinAlgNumber = SafeLinAlgNumber::from(32 as i64);
/// let var2 : SafeLinAlgNumber = SafeLinAlgNumber::from(0 as i64);
/// let var3 : SafeLinAlgNumber = SafeLinAlgNumber::from(-32 as i64);
/// let var4 : SafeLinAlgNumber = SafeLinAlgNumber::from(9007199254740992); //Max int representable in f64 + 1
/// let var5 : SafeLinAlgNumber = SafeLinAlgNumber::from(9007199254740991);
/// let var6 : SafeLinAlgNumber = SafeLinAlgNumber::from(-9007199254740991);
/// let var7 : SafeLinAlgNumber = SafeLinAlgNumber::from(-9007199254740992); //Yay 7
/// 
/// assert_eq!(var1, Safe(Float64(32.0)));
/// assert_eq!(var2, Safe(Float64(0.0)));
/// assert_eq!(var3, Safe(Float64(-32.0)));
/// assert_eq!(var4, SafeConversionIsImpossible("This integer is too big to be represented as f64 without precision loss".to_owned()));
/// assert_eq!(var5, Safe(Float64(9007199254740991.0)));
/// assert_eq!(var6, Safe(Float64(-9007199254740991.0)));
/// assert_eq!(var7, SafeConversionIsImpossible("This integer is too big to be represented as f64 without precision loss".to_owned()));
///  ```
impl From<i64> for SafeLinAlgNumber {
    fn from(value: i64) -> Self {
        if perfectly_representable_as_f64(&value) {
            SafeLinAlgNumber::Safe(LinAlgNumber::Float64(value as f64)) 
        } else {
            SafeConversionIsImpossible("This integer is too big to be represented as f64 without precision loss".to_owned())
        }
    }
}



//Impl Comparison traits
impl Eq for LinAlgNumber {}

//Partial Eq Gauntlet
/// This method takes care of comparing variants LinAlgNumber.
/// We take the decision to only allow comparison between identical types due to precision.
/// While at first this was implemented with casting from f32 to f64.
/// Further reflection lead to the realization that due to their nature, precision would be different.
/// If ones judges that the f64 precision is required.
/// Then it is fair to assume that 3.000000000 and 3.000000000000000000 while to us mortals the same thing,
/// has a different meaning. 
/// 
/// As such for precision purposes, and to not encourage weird behavior, comparison between different variants will 
/// always yield false.
/// 
/// It is expected that the user will pick one variant and stick to it.
/// 
/// # Examples
/// 
/// ```
/// use rustiest_linear_algebra::linear_algebra_number::LinAlgNumber::{Float64, Float32, NaN};
/// 
/// let var1 : bool = Float64(42.0) == Float64(42.0); //True
/// let var2 : bool = Float64(69.0) == Float64(-69.0); //False
/// let var3 : bool = Float32(f32::from(0.0)) == Float32(f32::from(0.0)); //True
/// let var4 : bool = Float32(f32::from(-2.0)) == Float32(f32::from(2.0)); //False
/// let var5 : bool = Float64(4.0) == Float32(4.0); //Everything else false, since we compare different variants.
/// let var6 : bool = Float32(4.0) == Float64(4.0);
/// let var7 : bool = Float64(-1.0) == NaN;
/// let var8 : bool = NaN == Float32(f32::from(1.0));
/// 
/// assert!(var1);
/// assert!(!var2);
/// assert!(var3);
/// assert!(!var4);
/// assert!(!var5);
/// assert!(!var6);
/// assert!(!var7);
/// assert!(!var8);
/// ```
impl PartialEq<LinAlgNumber> for LinAlgNumber {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            //Same type
            (Float64(value_self), Float64(value_other)) => value_self == value_other,
            (Float32(value_self), Float32(value_other)) => value_self == value_other,
            //Anything else
            (_, _) => false
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

