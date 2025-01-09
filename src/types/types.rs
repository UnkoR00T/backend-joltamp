use serde::Serialize;

#[derive(Serialize)]
pub struct RequestError{
    message: String,
}

impl<T: ToString> From<T> for RequestError where std::string::String: From<T>
{
    fn from(value: T) -> Self {
        RequestError {
            message: String::from(value),
        }
    }
}

impl RequestError {
    
}

#[derive(Serialize)]
pub struct ReturnUser {
    pub(crate) jwt: String,
    pub(crate) user_id: String,
}