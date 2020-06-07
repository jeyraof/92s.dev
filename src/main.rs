use rouille::{Response, ResponseBody, router};
use tera::{Tera, Context};
use mysql::Pool;
use std::env;

mod record;
mod errors;

fn main() -> std::result::Result<(), errors::Error> {
    let templates = get_templates()?;
    let pool = get_db_pool()?;

    rouille::start_server("0.0.0.0:8088", move |request| {
        router!(request,
            (GET) (/) => { record::handlers::fetch_recent_used(request, &templates, &pool) },
            (GET) (/{slug: String}) => { record::handlers::fetch_by_slug(request, &templates, &pool, slug) },
            (POST) (/new) => { record::handlers::create(request, &templates, &pool) },
            _ => response_by(&templates, 404)
        )
    });
}

fn get_templates() -> Result<Tera, errors::Error> {
    let mut templates = Tera::new("src/templates/**/*")?;
    templates.autoescape_on(vec!["html"]);
    Ok(templates)
}

fn get_db_pool() -> Result<Pool, errors::Error> {
    let database_url = env::var("DATABASE_URL")?;
    Ok(Pool::new(database_url)?)
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