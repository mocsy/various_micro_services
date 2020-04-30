use iron::prelude::*;
use iron::status;
use router::Router;
use serde::Serialize;
use serde_json::json;

use various_micro_services::{Create, Delete, Fetch, List, Replace, Todo, Update};

fn main() {
    let mut router = Router::new();

    router.get("todo/add", todo_add, "todo_add");
    router.get("todo/list", todo_list, "todo_list");
    router.get("todo/fetch/:todo_key", todo_fetch, "todo_fetch");
    router.get("todo/edit", todo_edit, "todo_edit");
    router.get("todo/replace", todo_replace, "todo_replace");
    router.get("todo/delete/:todo_key", todo_delete, "todo_delete");

    let _res = Iron::new(router).http("localhost:3000");
}

fn todo_add(request: &mut Request) -> Result<iron::response::Response, iron::error::IronError> {
    let content_type = "application/json".parse::<iron::mime::Mime>().unwrap();
    let json_body = request.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => match serde_json::from_value::<Todo>(json_body) {
            Ok(todo) => match Todo::create(todo) {
                Ok(resp) => Ok(Response::with((
                    content_type,
                    status::Ok,
                    serde_json::to_string(&resp).unwrap(),
                ))),
                Err(e) => Ok(Response::with((
                    content_type,
                    status::BadRequest,
                    serde_json::to_string(&logged_response("", &e, true)).unwrap(),
                ))),
            },
            Err(e) => Ok(Response::with((
                content_type,
                status::BadRequest,
                serde_json::to_string(&logged_response(&format!("{:?}", e), &json!(""), true))
                    .unwrap(),
            ))),
        },
        Ok(None) => Ok(Response::with((
            content_type,
            status::BadRequest,
            serde_json::to_string(&logged_response(
                "Couldn't parse request body.",
                &json!(""),
                true,
            ))
            .unwrap(),
        ))),
        Err(e) => Ok(Response::with((
            content_type,
            status::BadRequest,
            serde_json::to_string(&logged_response(&format!("{:?}", e), &json!(""), true)).unwrap(),
        ))),
    }
}

fn todo_list(request: &mut Request) -> Result<iron::response::Response, iron::error::IronError> {
    let content_type = "application/json".parse::<iron::mime::Mime>().unwrap();
    let limit = match request.url.query() {
        Some(ref query) => query.parse::<u64>().unwrap_or_else(|_| 100u64),
        None => 100u64,
    };
    match Todo::list(limit) {
        Ok(resp) => Ok(Response::with((
            content_type,
            status::Ok,
            serde_json::to_string(&resp).unwrap(),
        ))),
        Err(e) => Ok(Response::with((
            content_type,
            status::BadRequest,
            serde_json::to_string(&logged_response("", &e, true)).unwrap(),
        ))),
    }
}

fn todo_fetch(request: &mut Request) -> Result<iron::response::Response, iron::error::IronError> {
    let content_type = "application/json".parse::<iron::mime::Mime>().unwrap();
    if let Some(ref todo_key) = request.extensions.get::<Router>().unwrap().find("todo_key") {
        match Todo::fetch(todo_key) {
            Ok(resp) => Ok(Response::with((
                content_type,
                status::Ok,
                serde_json::to_string(&resp).unwrap(),
            ))),
            Err(e) => Ok(Response::with((
                content_type,
                status::BadRequest,
                serde_json::to_string(&logged_response("", &e, true)).unwrap(),
            ))),
        }
    } else {
        Ok(Response::with((
            content_type,
            status::BadRequest,
            serde_json::to_string(&logged_response("Missing todo _key.", &json!(""), true))
                .unwrap(),
        )))
    }
}

fn todo_edit(request: &mut Request) -> Result<iron::response::Response, iron::error::IronError> {
    let content_type = "application/json".parse::<iron::mime::Mime>().unwrap();
    let json_body = request.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => match Todo::update(json_body) {
            Ok(resp) => Ok(Response::with((
                content_type,
                status::Ok,
                serde_json::to_string(&resp).unwrap(),
            ))),
            Err(e) => Ok(Response::with((
                content_type,
                status::BadRequest,
                serde_json::to_string(&logged_response("", &e, true)).unwrap(),
            ))),
        },
        Ok(None) => Ok(Response::with((
            content_type,
            status::BadRequest,
            serde_json::to_string(&logged_response(
                "Couldn't parse request body.",
                &json!(""),
                true,
            ))
            .unwrap(),
        ))),
        Err(e) => Ok(Response::with((
            content_type,
            status::BadRequest,
            serde_json::to_string(&logged_response(&format!("{:?}", e), &json!(""), true)).unwrap(),
        ))),
    }
}

fn todo_replace(request: &mut Request) -> Result<iron::response::Response, iron::error::IronError> {
    let content_type = "application/json".parse::<iron::mime::Mime>().unwrap();
    let json_body = request.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => match serde_json::from_value::<Todo>(json_body) {
            Ok(todo) => match Todo::replace(todo) {
                Ok(resp) => Ok(Response::with((
                    content_type,
                    status::Ok,
                    serde_json::to_string(&resp).unwrap(),
                ))),
                Err(e) => Ok(Response::with((
                    content_type,
                    status::BadRequest,
                    serde_json::to_string(&logged_response("", &e, true)).unwrap(),
                ))),
            },
            Err(e) => Ok(Response::with((
                content_type,
                status::BadRequest,
                serde_json::to_string(&logged_response(&format!("{:?}", e), &json!(""), true))
                    .unwrap(),
            ))),
        },
        Ok(None) => Ok(Response::with((
            content_type,
            status::BadRequest,
            serde_json::to_string(&logged_response(
                "Couldn't parse request body.",
                &json!(""),
                true,
            ))
            .unwrap(),
        ))),
        Err(e) => Ok(Response::with((
            content_type,
            status::BadRequest,
            serde_json::to_string(&logged_response(&format!("{:?}", e), &json!(""), true)).unwrap(),
        ))),
    }
}

fn todo_delete(request: &mut Request) -> Result<iron::response::Response, iron::error::IronError> {
    let content_type = "application/json".parse::<iron::mime::Mime>().unwrap();
    if let Some(ref todo_key) = request.extensions.get::<Router>().unwrap().find("todo_key") {
        match Todo::delete(todo_key) {
            Ok(resp) => Ok(Response::with((
                content_type,
                status::Ok,
                serde_json::to_string(&resp).unwrap(),
            ))),
            Err(e) => Ok(Response::with((
                content_type,
                status::BadRequest,
                serde_json::to_string(&logged_response("", &e, true)).unwrap(),
            ))),
        }
    } else {
        Ok(Response::with((
            content_type,
            status::BadRequest,
            serde_json::to_string(&logged_response("Missing todo _key.", &json!(""), true))
                .unwrap(),
        )))
    }
}

fn logged_response<'t, T: Serialize + core::fmt::Debug>(
    msg: &'t str,
    param: &'t T,
    is_error: bool,
) -> serde_json::Value {
    let resp = json!({
        "is_error": is_error,
        "message": msg,
        "detail": param,
    });
    log::error!(
        "{}",
        serde_json::to_string(&resp).unwrap_or_else(|e| e.to_string())
    );
    resp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logged_response() {
        let num = 1588237987000i64;
        let exp = json!({
            "is_error": true,
            "message": "I'm a teapot.",
            "detail": {
                "_key":"",
                "title":"Write more tests",
                "timestamp":num,
                "status":"New"
            }
        });

        let mut todo = Todo::new("Write more tests");
        todo.back_date(&time::OffsetDateTime::from_unix_timestamp(1_588_237_987));
        let resp = logged_response("I'm a teapot.", &todo, true);
        assert_eq!(exp.to_string(), serde_json::to_string(&resp).unwrap());
    }
}
