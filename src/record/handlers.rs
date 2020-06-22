use crate::record::{Record, RecordJSON};
use crate::{response_by, json_error, ErrorResponse};
use crate::errors;
use rouille::{Request, Response, try_or_400};
use tera::{Tera, Context};
use mysql::Pool;

pub fn fetch_recent_used(request: &Request, templates: &Tera, pool: &Pool) -> Response {
    let count: i32 = request.get_param("count").unwrap_or(String::from("5")).parse().unwrap_or(5);
    let mut context = Context::new();
    let records = Record::fetch_last_used(&count, &pool).unwrap();
    context.insert("records", &records);
    let content = templates.render("index.html", &context).expect("render failed");
    Response::html(content)
}

pub fn fetch_by_slug(_request: &Request, templates: &Tera, pool: &Pool, slug: &String) -> Response {
    let record = Record::fetch_by_slug(&slug, &pool).unwrap();
    match record {
        Some(r) => {
            let _ = Record::update_last_used(&r.id, &pool);
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
    let creation = Record::create(&json, &pool);

    match creation {
        Ok(r) => Response::json(&json),
        Err(errors::NinetyTwoError::MySqlError(e)) => {
            match &json.overwrite {
                true => {
                    let updation = Record::update(&json, &pool);
                    match updation {
                        Ok(r) => Response::json(&json),
                        _ => json_error(&ErrorResponse{code: 500, http_status: 500})
                    }
                },
                false => json_error(&ErrorResponse{code: 409, http_status: 409})
            }
        }
        Err(e) => json_error(&ErrorResponse{code: 409, http_status: 409})
    }
}