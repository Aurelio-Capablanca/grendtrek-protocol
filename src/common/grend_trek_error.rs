use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum StopTrek {
    IO(std::io::Error),
    SQLx(sqlx::Error),
    Tiberius(tiberius::error::Error),
    JSON(serde_json::Error),
    CustomMessage(String)
}

impl From<std::io::Error> for StopTrek {
    fn from(err: std::io::Error) -> Self {
        StopTrek::IO(err)
    }
}

impl From<sqlx::Error> for StopTrek {
    fn from(err: sqlx::Error) -> Self {
        StopTrek::SQLx(err)
    }
}

impl From<tiberius::error::Error> for StopTrek {
    fn from(err: tiberius::error::Error) -> Self {
        StopTrek::Tiberius(err)
    }
}

impl From<serde_json::Error> for StopTrek {
    fn from (err: serde_json::Error) -> Self{
        StopTrek::JSON(err)
    }
}


use std::error::Error as StdError;
impl StdError for StopTrek {}
impl fmt::Display for StopTrek{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StopTrek::SQLx(e) => write!(f, "SQLx error: {}", e),
            StopTrek::Tiberius(e) => write!(f, "Tiberius error: {}", e),
            StopTrek::CustomMessage(msg) => write!(f, "Custom error: {}", msg),
            StopTrek::IO(e) => write!(f,"IO error: {}",e),
            StopTrek::JSON(e) => write!(f,"JSON error: {}",e)
        }
    }
}