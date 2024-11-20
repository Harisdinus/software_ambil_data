use serde::Serialize;

pub type Result<T,E = Error> = std::result::Result<T,E>;

#[derive(Serialize)]
pub struct Error {
    message: String
}

impl<E> From<E> for Error where E: std::error::Error {
    fn from(value: E) -> Self {
        eprintln!("Error: {value:#?}");
        Self {
            message: value.to_string()
        }
    }
}


pub struct Globals {
    pub url: String,
}

impl Globals {
    pub fn use_url(&self, path: &str) -> String {
        format!("{}{}",&self.url,path)
    }
}

impl Default for Globals {
    fn default() -> Self {
        Self {
            url: "http://localhost:3000".into(),
        }
    }
}
