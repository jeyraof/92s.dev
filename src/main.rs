use serde::Serialize;
use rouille::{Response, ResponseBody, router};
use tera::{Tera, Context};
use mysql::Pool;
use std::env;
use serde_json;

mod record;
mod errors;
use errors::Result;

fn main() -> Result<()> {
    let templates = get_templates()?;
    let pool = get_db_pool()?;

    rouille::start_server("0.0.0.0:8088", move |request| {
        router!(request,
            (GET) (/) => { record::handlers::fetch_recent_used(request, &templates, &pool) },
            (GET) (/{slug: String}) => { record::handlers::fetch_by_slug(request, &templates, &pool, &slug) },
            (POST) (/new) => { record::handlers::create(request, &templates, &pool) },
            _ => response_by(&templates, 404)
        )
    });
}

fn get_templates() -> Result<Tera> {
    let mut templates = Tera::new("src/templates/**/*").expect("No template provided");
    templates.autoescape_on(vec!["html"]);
    Ok(templates)
}

fn get_db_pool() -> Result<Pool> {
    let database_url = env::var("DATABASE_URL").expect("`DATABASE_URL` must be set");
    let pool = Pool::new(database_url).expect("Failed to connect with database");
    Ok(pool)
}

pub fn response_by(templates: &Tera, code: i32) -> Response {
    let context = Context::new();
    let content = templates.render(&format!("errors/{}.html", code), &context).unwrap();
    Response {
        status_code: 404,
        headers: vec![("Content-Type".into(), "text/html; charset=utf8".into())],
        data: ResponseBody::from_string(content),
        upgrade: None,
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub http_status: u16,
}
pub fn json_error(content: &ErrorResponse) -> Response {
    let data = serde_json::to_string(content).unwrap();

    Response {
        status_code: content.http_status,
        headers: vec![("Content-Type".into(), "application/json".into())],
        data: ResponseBody::from_data(data),
        upgrade: None,
    }
}