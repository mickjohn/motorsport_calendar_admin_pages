use std;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IoError")]
    IoError(std::io::Error),
    #[fail(display = "Deserialisation Error")]
    DeserialisationError(toml::de::Error),
}

impl From<toml::de::Error> for Error {
    fn from(toml_error: toml::de::Error) -> Self {
        Error::DeserialisationError(toml_error)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Error::IoError(io_error)
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_server_host: String,
    pub api_server_port: i32,
    pub content_root: String,
    pub css_root: String,
    pub js_root: String,
}

impl Config {
    pub fn from_toml_file(path: &Path) -> Result<Self, Error> {
        let mut f = File::open(path)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        Ok(toml::from_str(&contents)?)
    }
}
