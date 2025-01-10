use serde::Serialize;

/// Custom universal error type for returning an error message via RESTful API.
#[derive(Serialize)]
pub struct RequestError{
    error: String,
}

// Implementaion of From trait for ::from(String) usage
impl<T: ToString> From<T> for RequestError where std::string::String: From<T>
{
    fn from(value: T) -> Self {
        RequestError {
            error: String::from(value),
        }
    }
}