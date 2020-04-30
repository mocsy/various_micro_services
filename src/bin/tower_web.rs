use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use shrinkwraprs::Shrinkwrap;
use tower_web::{
    derive_resource, derive_resource_impl, impl_web, impl_web_clean_nested,
    impl_web_clean_top_level, Extract, Response, ServiceBuilder,
};
use various_micro_services as vms;
use vms::{Create, Delete, Fetch, List, Replace, Update};

/// This type will be part of the web service as a resource.
#[derive(Debug)]
struct HelloWorld;

/// This will be the JSON response
#[derive(Response)]
struct HelloResponse {
    message: &'static str,
}

#[derive(Shrinkwrap, Debug, Response, Serialize, Extract)]
// #[shrinkwrap(mutable)]
// #[shrinkwrap(transformers)]
struct Todo(vms::Todo);
impl From<vms::Todo> for Todo {
    fn from(vtd: vms::Todo) -> Self {
        Todo(vtd)
    }
}
impl Todo {
    pub(crate) fn map_vec(vtd: Vec<vms::Todo>) -> Vec<Todo> {
        let mut res = vec![];
        for item in vtd {
            res.push(Todo(item));
        }
        res
    }
}

#[derive(Shrinkwrap, Debug, Extract)]
struct ListOptions(vms::ListOptions);

impl_web! {
    impl HelloWorld {
        #[get("/todo/list")]
        #[content_type("json")]
        fn todo_list(&self, query_string: ListOptions) -> Result<Vec<Todo>, Value> {
            match vms::Todo::list(query_string.limit.unwrap_or_else(|| 100u64)) {
                Ok(resp) => {
                    let res: Vec<Todo> = Todo::map_vec(resp);
                    Ok(res)
                },
                Err(e) => Err(e),
            }
        }

        #[get("/todo/fetch/:todo_key")]
        #[content_type("json")]
        fn todo_fetch(&self, todo_key: String) -> Result<Todo, Value> {
            match vms::Todo::fetch(&todo_key) {
                Ok(resp) => Ok(Todo(resp)),
                Err(e) => Err(e),
            }
        }

        #[get("/todo/create")]
        #[content_type("json")]
        fn todo_create(&self, body: Todo) -> Result<Todo, Value> {
            match vms::Todo::create(body.0) {
                Ok(resp) => Ok(Todo(resp)),
                Err(e) => Err(e),
            }
        }

        #[get("/todo/update")]
        #[content_type("json")]
        fn todo_update(&self, body: serde_json::Value) -> Result<Todo, Value> {
            match vms::Todo::update(body) {
                Ok(resp) => Ok(Todo(resp)),
                Err(e) => Err(e),
            }
        }

        #[get("/todo/replace")]
        #[content_type("json")]
        fn todo_replace(&self, body: Todo) -> Result<Todo, Value> {
            match vms::Todo::replace(body.0) {
                Ok(resp) => Ok(Todo(resp)),
                Err(e) => Err(e),
            }
        }

        #[get("/todo/delete/:todo_key")]
        #[content_type("json")]
        fn todo_delete(&self, todo_key: String) -> Result<Todo, Value> {
            match vms::Todo::delete(&todo_key) {
                Ok(resp) => Ok(Todo(resp)),
                Err(e) => Err(e),
            }
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HelloWorld)
        .run(&addr)
        .unwrap();
}
