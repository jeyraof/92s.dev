#[derive(thiserror::Error, Debug)]
pub enum NinetyTwoError {
    // main


    // record
    #[error("Record already exists")]
    RecordAlreadyExist,

    // authentication
    #[error("access token not found")]
    AccessTokenNotFound,
    #[error("access token expired")]
    AccessTokenExpired,
    #[error("invalid access token")]
    InvalidAccessToken,

    #[error("refresh token not found")]
    RefreshTokenNotFound,
    #[error("refresh token expired")]
    RefreshTokenExpired,
    #[error("invalid refresh token")]
    InvalidRefreshToken,

    // mysql
    #[error("mysql error: {0}")]
    MySqlError(#[from] mysql::error::Error),
}

pub type Result<T> = std::result::Result<T, NinetyTwoError>;
