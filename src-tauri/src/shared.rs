use serde::Serialize;

pub type Result<T,E = Error> = std::result::Result<T,E>;

#[derive(Serialize)]
pub struct Error {
    message: String
}

impl<E> From<E> for Error where E: std::error::Error {
    fn from(value: E) -> Self {
        Self {
            message: value.to_string()
        }
    }
}

