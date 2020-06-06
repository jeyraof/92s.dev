use rouille::{Response, ResponseBody, router};
use tera::{Tera, Context};
use mysql::{Pool, FromRowError, Row};
use mysql::prelude::*;
use std::env;
use serde::Serialize;

#[derive(Serialize)]
struct Record {
    id: i32,
    slug: String,
    url: Option<String>,
}

impl FromRow for Record {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        let (id, slug, url) = FromRow::from_row_opt(row)?;
        Ok(Self { id, slug, url })
    }
}

type Error = Box<dyn std::error::Error>;

fn response_by(templates: &Tera, code: i32) -> Response {
    // templates.get_template(template_name)
    let context = Context::new();
    let content = templates.render(&format!("errors/{}.html", code), &context).unwrap();
    Response {
        status_code: 404,
        headers: vec![("Content-Type".into(), "text/html; charset=utf8".into())],
        data: ResponseBody::from_string(content),
        upgrade: None,
    }
}

fn main() -> std::result::Result<(), Error> {
    let templates = get_templates()?;
    let pool = get_database()?;

    rouille::start_server("0.0.0.0:8088", move |request| {
        router!(request,
            (GET) (/) => {
                let mut context = Context::new();
                let records = get_records(&pool).unwrap();

                context.insert("slugs", &records);
                let content = templates.render("index.html", &context).expect("render failed");
                Response::html(content)
            },
            (GET) (/{slug: String}) => {
                let record = get_record_by(slug, &pool).unwrap();
                match record {
                    Some(r) => {
                        let mut context = Context::new();
                        context.insert("record", &r);
                        let content = templates.render("loading.html", &context).expect("render failed");
                        Response::html(content)
                    },
                    None => response_by(&templates, 404)
                }

            },
            _ => response_by(&templates, 404)
        )
    });
}

fn get_templates() -> Result<Tera, Error> {
    let mut templates = Tera::new("src/templates/**/*")?;
    templates.autoescape_on(vec!["html"]);
    Ok(templates)
}

fn get_database() -> Result<Pool, Error> {
    let database_url = env::var("DATABASE_URL")?;
    Ok(Pool::new(database_url)?)
}

fn get_records(pool: &Pool) -> Result<Vec<Record>, Error> {
    let mut conn = pool.get_conn()?;
    let items = conn.query_map(
        r#"
        SELECT id, slug, url 
        FROM records
        ORDER BY id DESC
        LIMIT 5;
        "#, 
        |(id, slug, url)| {
            Record { id, slug, url }
        }
    ).unwrap();
    Ok(items)
}

fn get_record_by(slug: String, pool: &Pool) -> Result<Option<Record>, Error> {
    let mut conn = pool.get_conn()?;
    let record: Option<Record> = conn.query_first(
        format!(
            r#"
            SELECT id, slug, url
            FROM records
            WHERE slug = '{}'
            LIMIT 1
            "#,
            slug
        )
    ).unwrap();
    Ok(record)
}