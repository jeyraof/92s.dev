#[derive(thiserror::Error, Debug)]
pub enum NinetyTwoError {
    // main


    // record
    #[error("Record already exists")]
    RecordAlreadyExist,

    // authentication
    #[error("access token expired")]
    AccessTokenExpired,
    #[error("invalid access token")]
    InvalidAccessToken,
    #[error("refresh token expired")]
    RefreshTokenExpired,
    #[error("invalid refresh token")]
    InvalidRefreshToken,

    // mysql
    #[error("mysql error: {0}")]
    MySqlError(#[from] mysql::error::Error),

    #[error("mysql error: {0}")]
    MysqlError(#[from] mysql::error::MySqlError),
}

pub type Result<T> = std::result::Result<T, NinetyTwoError>;
