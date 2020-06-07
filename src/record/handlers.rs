use crate::record::{Record, RecordJSON};
use crate::response_by;
use rouille::{Request, Response, try_or_400};
use tera::{Tera, Context};
use mysql::Pool;

pub fn fetch_recent_used(request: &Request, templates: &Tera, pool: &Pool) -> Response {
    let count: i32 = request.get_param("count").unwrap_or(String::from("5")).parse().unwrap_or(5);
    let mut context = Context::new();
    let records = Record::fetch_last_used(count, &pool).unwrap();
    context.insert("records", &records);
    let content = templates.render("index.html", &context).expect("render failed");
    Response::html(content)
}

pub fn fetch_by_slug(_request: &Request, templates: &Tera, pool: &Pool, slug: String) -> Response {
    let record = Record::fetch_by_slug(slug, &pool).unwrap();
    match record {
        Some(r) => {
            let _ = Record::update_last_used(r.id, &pool);
            let mut context = Context::new();
            context.insert("record", &r);
            let content = templates.render("loading.html", &context).expect("render failed");
            Response::html(content)
        },
        None => response_by(&templates, 404)
    }
}

pub fn create(request: &Request, templates: &Tera, pool: &Pool) -> Response {
    let json: RecordJSON = try_or_400!(rouille::input::json_input(request));
    let record = Record::create(&json, &pool).unwrap();
    match record {
        Some(r) => Response::redirect_302(format!("/{}", r.slug)),
        None => response_by(&templates, 404)
    }
}