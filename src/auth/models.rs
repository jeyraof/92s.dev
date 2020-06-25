use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Utc};
use mysql::{Pool, Row, FromRowError};
use mysql::prelude::*;
use uuid::Uuid;
use crate::errors::{Result, NinetyTwoError};
use mysql_common::params::Params;


trait Token {
    fn is_expired(&self) -> bool;
}

trait Refreshable {
    fn refresh(&self) -> Self;
}

// Access Token
#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    pub id: i32,
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub refresh_token_id: i32
}

impl FromRow for AccessToken {
    fn from_row_opt(row: Row) -> std::result::Result<Self, FromRowError> {
        let (id, token, expires_at, created_at, updated_at, refresh_token_id) = FromRow::from_row_opt(row)?;
        Ok(Self { id, token, expires_at, created_at, updated_at, refresh_token_id })
    }
}

impl Token for AccessToken {
    fn is_expired(&self) -> bool {
        self.expires_at < Utc::now().naive_utc()
    }
}

impl AccessToken {
    pub fn new(refresh_token_id: &i32, pool: &Pool) -> Result<AccessToken> {
        let mut conn = pool.get_conn()?;
        let stmt = "INSERT INTO access_tokens (token, expires_at, refresh_token_id) values (?, ?, ?)";

        let refresh_token = RefreshToken::fetch_by_id(refresh_token_id, pool);
        let real_rt_id: i32;
        match refresh_token {
            Ok(rt) => { real_rt_id = rt.id },
            Err(e) => { return Err(e) }
        }

        loop {
            match conn.exec_first(&stmt, (Uuid::new_v4().to_string(), Utc::now().naive_utc(), real_rt_id)) {
                Ok(Some(at)) => { return Ok(at) },
                Ok(None) => { continue },
                Err(_) => { continue }
            }
        }
    }

    pub fn fetch_by_token(token: &String, pool: &Pool) -> Result<AccessToken> {
        let mut conn = pool.get_conn()?;
        let fetched: std::result::Result<Option<AccessToken>, mysql::error::Error> = conn.exec_first(
            "SELECT id, token, expires_at, created_at, updated_at, refresh_token_id FROM access_tokens where token = ?",
            (token, )
        );

        match fetched {
            Ok(Some(at)) => {
                if at.is_expired() { return Err(NinetyTwoError::AccessTokenExpired) }
                Ok(at)
            }
            Ok(None) => Err(NinetyTwoError::AccessTokenNotFound),
            Err(_) => Err(NinetyTwoError::InvalidAccessToken),
        }
    }
}

// Refresh Token
#[derive(Serialize, Deserialize)]
pub struct RefreshToken {
    pub id: i32,
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl FromRow for RefreshToken {
    fn from_row_opt(row: Row) -> std::result::Result<Self, FromRowError> {
        let (id, token, expires_at, created_at, updated_at ) = FromRow::from_row_opt(row)?;
        Ok(Self { id, token, expires_at, created_at, updated_at })
    }
}

impl Token for RefreshToken {
    fn is_expired(&self) -> bool {
        self.expires_at < Utc::now().naive_utc()
    }
}

impl RefreshToken {
    pub fn new(pool: &Pool) -> Result<RefreshToken> {
        let mut conn = pool.get_conn()?;
        let stmt = "INSERT INTO refresh_tokens (token, expires_at) values (?, ?)";

        loop {
            match conn.exec_first(&stmt, (Uuid::new_v4().to_string(), Utc::now().naive_utc())) {
                Ok(Some(rt)) => { return Ok(rt) },
                Ok(None) => { continue },
                Err(_) => { continue },
            }
        }
    }

    pub fn fetch_by_id(id: &i32, pool: &Pool) -> Result<RefreshToken> {
        let mut conn = pool.get_conn()?;
        let fetched: std::result::Result<Option<RefreshToken>, mysql::error::Error> = conn.exec_first(
            "SELECT id, token, expires_at, created_at, updated_at FROM refresh_tokens where id = ?",
            (id, )
        );
        match fetched {
            Ok(Some(rt)) => {
                if rt.is_expired() { return Err(NinetyTwoError::RefreshTokenExpired) }
                Ok(rt)
            }
            Ok(None) => Err(NinetyTwoError::RefreshTokenNotFound),
            Err(_) => Err(NinetyTwoError::InvalidAccessToken),
        }
    }

    pub fn fetch_by_token(token: &String, pool: &Pool) -> Result<RefreshToken> {
        let mut conn = pool.get_conn()?;
        let fetched: std::result::Result<Option<RefreshToken>, mysql::error::Error> = conn.exec_first(
            "SELECT id, token, expires_at, created_at, updated_at FROM refresh_tokens where token = ?",
            (token, )
        );

        match fetched {
            Ok(Some(rt)) => {
                if rt.is_expired() { return Err(NinetyTwoError::RefreshTokenExpired) }
                Ok(rt)
            }
            Ok(None) => Err(NinetyTwoError::RefreshTokenNotFound),
            Err(_) => Err(NinetyTwoError::InvalidAccessToken),
        }
    }
}