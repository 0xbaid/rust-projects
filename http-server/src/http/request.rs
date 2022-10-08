use super::method::Method;
pub struct Request {
    path: String,
    query_string: Option<String>, //wrapping query_string in option enum to handle if there's no value passed
    method: Method,
}
