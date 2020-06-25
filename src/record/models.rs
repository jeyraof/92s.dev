use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Utc};
use mysql::{Pool, Row, FromRowError};
use mysql::prelude::*;
// use anyhow::Result;
// use crate::errors::NinetyTwoError;
use crate::errors::Result;

#[derive(Serialize, Deserialize)]
pub struct Record {
    pub id: i32,
    pub slug: String,
    pub url: String,
    pub created_at: NaiveDateTime,
    pub last_used_at: Option<NaiveDateTime>,
}

impl FromRow for Record {
    fn from_row_opt(row: Row) -> std::result::Result<Self, FromRowError> {
        let (id, slug, url, created_at, last_used_at) = FromRow::from_row_opt(row)?;
        Ok(Self { id, slug, url, created_at, last_used_at })
    }
}

#[derive(Serialize, Deserialize)]
pub struct RecordJSON {
    pub slug: String,
    pub url: String,
    pub overwrite: bool,
}

impl Record {
    pub fn create(json: &RecordJSON, pool: &Pool) -> Result<Option<Record>> {
        let mut conn = pool.get_conn()?;

        let creation = conn.exec_first(
            "INSERT INTO records (slug, url) VALUES (?, ?)",
            (&json.slug, &json.url),
        );

        match creation {
            Ok(r) => Ok(r),
            Err(e) => Err(e.into())
        }
    }
    pub fn update(json: &RecordJSON, pool: &Pool) -> Result<Option<Record>> {
        let mut conn = pool.get_conn()?;
        let update = conn.exec_first(
            "UPDATE records SET url = ? WHERE slug = ?",
            (&json.url, &json.slug),
        );

        match update {
            Ok(r) => Ok(r),
            Err(e) => Err(e.into())
        }
    }
    pub fn fetch_last_used(count: &i32, pool: &Pool) -> Result<Vec<Record>> {
        let mut conn = pool.get_conn()?;
        let items: Vec<Record> = conn.exec(
            "SELECT id, slug, url, created_at, last_used_at FROM records USE INDEX(records_last_used_at_id_index) ORDER BY last_used_at DESC, id DESC LIMIT ?",
            (&count, ),
        ).unwrap();
        Ok(items)
    }
    pub fn fetch_by_slug(slug: &String, pool: &Pool) -> Result<Option<Record>> {
        let mut conn = pool.get_conn()?;
        let record: Option<Record> = conn.exec_first(
            "SELECT id, slug, url, created_at, last_used_at FROM records WHERE slug = ? LIMIT 1",
            (&slug, ),
        ).unwrap();
        Ok(record)
    }
    pub fn update_last_used(id: &i32, pool: &Pool) -> Result<Option<Record>> {
        let mut conn = pool.get_conn().unwrap();
        let record: Option<Record> = conn.exec_first(
            "UPDATE records SET last_used_at = ? WHERE id = ?",
            (Utc::now().naive_utc(), &id),
        ).unwrap();
        Ok(record)
    }
}

