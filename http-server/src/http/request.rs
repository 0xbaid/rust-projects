use crate::http::request;

use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>, //wrapping query_string in option enum to handle if there's no value passed
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1
    //converting byte slice to string slice
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        //above we convert error to parse error, other way is to directly use ? and imp From for parse error
        // as ? will try to convert utf8Error in to parseError by from trait implementation
        let request = str::from_utf8(buf)?;

        //variable shadowing, overwriting variable request with the remaining part
        //after extracting parts from url on the basis of space
        //every we create new tuple with request, old value of request become unusable
        //ok_or turns option enum to result enum, on success wraps value in ok(()) otherwise returns
        //error which is passed in ok_or

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        //parse comes where we implemented from_str trait
        //our function returns parse error but parse returns method error
        //we need to convert it using From trait
        let method: Method = method.parse()?;

        let mut query_string = None;
        //use if let statement becasue we care about the some variant
        //if let matches the pattern unwraps the result
        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]);
            path = &path[..i]
        }
        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        ParseError::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl Error for ParseError {}
