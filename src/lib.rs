use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::Debug,
    fs,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, Query, Request, State},
    http,
    http::StatusCode,
    middleware::Next,
    response::{Html, Response},
    Json,
};
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct AppState {
    pub counter: Arc<Mutex<u8>>,
    pub v: Arc<Mutex<Vec<String>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
            v: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

pub async fn index() -> Html<String> {
    // "Hello, World!"
    let html = fs::read_to_string("index.html").unwrap();
    Html(html)
}

// foo
pub async fn get_foo() -> &'static str {
    "foo"
}

pub async fn post_foo() -> &'static str {
    "POST foo"
}

// -----------
// extractor
// -----------
// `Path` gives you the path parameters and deserializes them.
pub async fn user_get(Path(user_id): Path<String>) -> String {
    println!("user_id: {}", user_id);
    format!("user id: {}", user_id)
}

// `Query` gives you the query parameters and deserializes them.
pub async fn query(Query(params): Query<HashMap<String, String>>) {
    println!("params: {:#?}", params);
}

// Buffer the request body and deserialize it as JSON into a
// `serde_json::Value`. `Json` supports any type that implements
// `serde::Deserialize`.
pub async fn json(Json(payload): Json<serde_json::Value>) {
    println!("payload: {:#?}", payload);
}

// -----------
// responses
// -----------
// `Json` gives a content-type of `application/json` and works with any type that implements `serde::Serialize`
pub async fn res_json() -> Json<Value> {
    Json(json!({"data":123, "messsage": "hola que tal, este es un json"}))
}

// -----------
// shared states
// -----------
pub async fn handler_state(State(state): State<AppState>) -> String {
    let counter = state.counter.lock().expect("lock counter");
    let v = state.v.lock().expect("lock v");

    println!("coutner: {}. vector: {:#?}", counter, v);

    // println!("coutner: {}. vector: {:#?}", state.counter, state.v);
    format!("coutner: {}. vector: {:#?}", counter, v)
}

pub async fn handler_state_post(State(state): State<AppState>) -> Json<Value> {
    let mut counter = state.counter.lock().expect("lock counter");
    *counter += 1;
    let mut v = state.v.lock().expect("lock v");
    v.push(format!("se agrego {}", counter));

    println!("coutner: {}. vector: {:#?}", counter, v);

    Json(json!({"counter": *counter, "vector": *v}))
}

// ------------
// middlewares
// ------------

#[derive(Clone)]
struct CurrentUser {
    id: u64,
}

pub async fn middleware_one(req: Request, next: Next) -> Result<Response, StatusCode> {
    println!("middleware_one");
    println!("req: {:#?}", req);
    // let headers = req.headers();
    // println!("headers: {:#?}", headers);
    // let path = req.uri();
    // println!("path: {:#?}", path);
    // let method = req.method();
    // println!("method: {:#?}", method);

    Ok(next.run(req).await)
}

pub async fn middleware_two(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    println!("middleware_two");
    println!("req: {:#?}", req);
    // let headers = req.headers();
    // println!("headers: {:#?}", headers);
    // let path = req.uri();
    // println!("path: {:#?}", path);
    // let method = req.method();
    // println!("method: {:#?}", method);

    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = match auth_header {
        Some(ah) => ah,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if let Some(current_user) = authorize_current_user(auth_header).await {
        // insert the current user into a request extension so the handler can
        // extract it
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    // ...
    None
}
