use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
pub struct Request {
    path: String,
    query_string: Option<String>, //wrapping query_string in option enum to handle if there's no value passed
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    //converting byte slice to string slice
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        //above we convert error to parse error, other way is to directly use ? and imp From for parse error
        // as ? will try to convert utf8Error in to parseError by from trait implementation
        let request = str::from_utf8(buf)?;

        unimplemented!();
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "Invalid Request",
            ParseError::InvalidEncoding => "Invalid Encoding",
            ParseError::InvalidProtocol => "Invalid Protocol",
            ParseError::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl Error for ParseError {}
