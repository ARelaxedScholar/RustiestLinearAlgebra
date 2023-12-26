use crate::linear_algebra_number::LinAlgNumber;


struct Vector<const SIZE:usize>{
    coordinates: [LinAlgNumber;SIZE],
    norm:LinAlgNumber
}

//Instance Methods
impl<const SIZE:usize> Vector<SIZE>{

}

//Class Methods
impl<const SIZE:usize> Vector<SIZE>{
    // fn new<T:Into<LinAlgNumber>>(coodinates:[T;SIZE]) -> Self{
    //     //Convert the coordinates to LinAlgNumber
    //     let wrapped_coordinates = coodinates.iter().map(|element| element.into()).collect();
    //     Vector { coordinates: wrapped_coordinates, norm: LinAlgNumber::Float64(0.0) }
    // }

    // fn default() -> Self{
    //     Vector { coordinates: [LinAlgNumber::from(0.0); SIZE], norm: LinAlgNumber::from(0.0) }
    // }

    // fn compute_norm(coordinates:[LinAlgNumber;SIZE]) -> LinAlgNumber{
    // }

}