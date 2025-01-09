use serde::Serialize;

/// Custom universal error type for returning an error message via RESTful API.
#[derive(Serialize)]
pub struct RequestError{
    message: String,
}

// Implementaion of From trait for ::from(String) usage
impl<T: ToString> From<T> for RequestError where std::string::String: From<T>
{
    fn from(value: T) -> Self {
        RequestError {
            message: String::from(value),
        }
    }
}


/// Custom return type for login and registration
#[derive(Serialize)]
pub struct ReturnUser {
    pub(crate) jwt: String,
    pub(crate) user_id: String,
}