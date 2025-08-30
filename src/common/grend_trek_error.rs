use std::io::Error;

#[derive(Debug)]
pub enum StopTrek {
    IO(std::io::Error),
    SQLx(sqlx::Error),
    Tiberius(tiberius::error::Error),
    JSON(serde_json::Error),
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
