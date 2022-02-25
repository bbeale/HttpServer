use std::fmt::{Display, Formatter, Result as FmtResult};


#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    // Redirect = 300,
    BadRequest = 400,
    NotFound = 404,
    // ServerError = 500
}
//TODO: Implement the rest of the HTTP statuses

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}