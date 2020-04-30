#[tokio::main]
async fn main() {
    let routes = filters::todo();

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

mod handlers {
    use std::convert::Infallible;
    use various_micro_services::{Create, Delete, Fetch, List, ListOptions, Replace, Todo, Update};

    pub async fn todo_list(opts: ListOptions) -> Result<impl warp::Reply, Infallible> {
        match Todo::list(opts.limit.unwrap_or_else(|| 100u64)) {
            Ok(resp) => Ok(warp::reply::json(&resp)),
            Err(e) => Ok(warp::reply::json(&e)),
        }
    }

    pub async fn todo_fetch(todo_key: String) -> Result<impl warp::Reply, Infallible> {
        match Todo::fetch(&todo_key) {
            Ok(resp) => Ok(warp::reply::json(&resp)),
            Err(e) => Ok(warp::reply::json(&e)),
        }
    }

    pub async fn todo_create(todo: Todo) -> Result<impl warp::Reply, Infallible> {
        match Todo::create(todo) {
            Ok(resp) => Ok(warp::reply::json(&resp)),
            Err(e) => Ok(warp::reply::json(&e)),
        }
    }

    pub async fn todo_update(
        todo_patch: serde_json::Value,
    ) -> Result<impl warp::Reply, Infallible> {
        match Todo::update(todo_patch) {
            Ok(resp) => Ok(warp::reply::json(&resp)),
            Err(e) => Ok(warp::reply::json(&e)),
        }
    }

    pub async fn todo_replace(todo: Todo) -> Result<impl warp::Reply, Infallible> {
        match Todo::replace(todo) {
            Ok(resp) => Ok(warp::reply::json(&resp)),
            Err(e) => Ok(warp::reply::json(&e)),
        }
    }

    pub async fn todo_delete(todo_key: String) -> Result<impl warp::Reply, Infallible> {
        match Todo::delete(&todo_key) {
            Ok(resp) => Ok(warp::reply::json(&resp)),
            Err(e) => Ok(warp::reply::json(&e)),
        }
    }
}

mod filters {
    use super::handlers;
    use various_micro_services::{ListOptions, Todo};
    use warp::Filter;

    /// The 6 Todo api filters combined.
    pub fn todo() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("todo").and(
            todo_list()
                .or(todo_fetch())
                .or(todo_create())
                .or(todo_update())
                .or(todo_replace())
                .or(todo_delete()),
        )
    }

    /// GET /todo/list/?offset=3&limit=5
    pub fn todo_list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("list")
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and_then(handlers::todo_list)
    }

    /// GET /todo/fetch/:todo_key
    pub fn todo_fetch() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("fetch" / String)
            .and(warp::get())
            .and_then(handlers::todo_fetch)
    }

    /// POST /todo/create with JSON body
    pub fn todo_create() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("create")
            .and(warp::post())
            .and(json_body())
            .and_then(handlers::todo_create)
    }

    /// PATCH /todo/update with JSON body
    pub fn todo_update() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("update")
            .and(warp::patch())
            .and(json_value_body())
            .and_then(handlers::todo_update)
    }

    /// PUT /todo/replace with JSON body
    pub fn todo_replace() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("replace")
            .and(warp::put())
            .and(json_body())
            .and_then(handlers::todo_replace)
    }

    /// DELETE /todo/delete/:todo_key
    pub fn todo_delete() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("delete" / String)
            .and(warp::delete())
            .and_then(handlers::todo_delete)
    }

    fn json_body() -> impl Filter<Extract = (Todo,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    fn json_value_body(
    ) -> impl Filter<Extract = (serde_json::Value,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}
