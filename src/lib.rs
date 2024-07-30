use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Method},
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

pub async fn index() -> String {
    // "Hello, World!"
    let html = fs::read_to_string("index.html").unwrap();
    html
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
pub async fn user_get(Path(user_id): Path<u32>) {
    println!("user_id: {}", user_id);
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

#[derive(Debug)]
struct BodyJson {
    json: String,
    name: String,
    age: u128,
}
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
