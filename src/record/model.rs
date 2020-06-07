use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Utc};
use mysql::{Pool, FromRowError, Row};
use mysql::prelude::*;
use mysql::error::Error as MysqlError;
use crate::errors::Error;

#[derive(Serialize, Deserialize)]
pub struct Record {
    pub id: i32,
    pub slug: String,
    pub url: String,
    pub created_at: NaiveDateTime,
    pub last_used_at: Option<NaiveDateTime>,
}

impl FromRow for Record {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        let (id, slug, url, created_at, last_used_at) = FromRow::from_row_opt(row)?;
        Ok(Self { id, slug, url, created_at, last_used_at })
    }
}

#[derive(Deserialize)]
pub struct RecordJSON {
    pub slug: String,
    pub url: String,
    pub force: bool,
}

impl Record {
    pub fn create(json: &RecordJSON, pool: &Pool) -> Result<Option<Record>, Error> {
        let mut conn = pool.get_conn()?;

        let record: Result<Option<Record>, MysqlError> = conn.query_first(
            format!(
                r#"
                INSERT INTO
                records (slug, url)
                VALUES ('{}', '{}')
                "#,
                json.slug,
                json.url,
            )
        );

        // TODO: 에러 종류에 따라 핸들링을 다르게 적용. (커스텀 에러를 만들어야 할 것 같다.)
        match record {
            Ok(r) => Ok(r),
            Err(MysqlError::MySqlError(_me)) => {
                let r = Record::fetch_by_slug(json.slug.clone(), &pool).unwrap();
                return Ok(r)
            },
            Err(_) => Ok(None)
        }
    }
    pub fn fetch_last_used(count: i32, pool: &Pool) -> Result<Vec<Record>, Error> {
        let mut conn = pool.get_conn()?;
        let items: Vec<Record> = conn.query(
            format!(
                r#"
                SELECT id, slug, url, created_at, last_used_at
                FROM records
                USE INDEX(records_last_used_at_id_index)
                ORDER BY 
                last_used_at DESC,
                id DESC
                LIMIT {};
                "#,
                count
            )
        ).unwrap();
        Ok(items)
    }
    pub fn fetch_by_slug(slug: String, pool: &Pool) -> Result<Option<Record>, Error> {
        let mut conn = pool.get_conn()?;
        let record: Option<Record> = conn.query_first(
            format!(
                r#"
                SELECT id, slug, url, created_at, last_used_at
                FROM records
                WHERE slug = '{}'
                LIMIT 1
                "#,
                slug
            )
        ).unwrap();
        Ok(record)
    }
    pub fn update_last_used(id: i32, pool: &Pool) -> Result<Option<Record>, Error> {
        let mut conn = pool.get_conn().unwrap();
        let record: Option<Record> = conn.query_first(
            format!(
                r#"
                UPDATE records
                SET last_used_at = '{}'
                WHERE id = {}
                "#,
                Utc::now().naive_utc(),
                id,
            )
        ).unwrap();
        Ok(record)
    }
}