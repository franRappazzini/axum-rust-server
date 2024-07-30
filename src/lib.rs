use std::{collections::HashMap, fs};

use axum::{
    extract::{Path, Query},
    Json,
};
use serde_json::{json, Value};

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
