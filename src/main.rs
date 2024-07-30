use axum::{routing::get, Router};
use axum_server::{get_foo, index, post_foo, res_json, user_get};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/user/{user_id}", get(user_get))
        .route("/json", get(res_json));

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "7878".to_string());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    println!("Server running on http://{}:{}", host, port);

    axum::serve(listener, app).await.unwrap();
}
