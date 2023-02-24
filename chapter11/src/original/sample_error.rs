use core::fmt;
use std::fmt::Display;
pub enum SampleError {
    IOError(std::io::Error) ,
    ParseError(std::num::ParseIntError)
}
impl Display for SampleError {
    fn fmt(&self , f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SampleError::IOError(error)=> write!(f , "IOError: {}" ,error) ,
            SampleError::ParseError(error) => write!(f , "ParseError: {}" ,error)
        }
    }
}
