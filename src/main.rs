use axum::{middleware, routing::get, Router};
use axum_server::{
    get_foo, handler_state, handler_state_post, index, middleware_one, middleware_two, post_foo,
    res_json, user_get, AppState,
};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    // shared state
    let shared_state = AppState::new();

    // let service_builder = ServiceBuilder::new()
    //     .layer(middleware::from_fn(middleware_one))
    //     .layer(middleware::from_fn(middleware_two));

    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        // .layer(service_builder)
        .route("/foo", get(get_foo).post(post_foo))
        // .layer(middleware::from_fn(middleware_one))
        .route("/user/:user_id", get(user_get))
        // .layer(middleware::from_fn(middleware_two))
        .route("/json", get(res_json))
        .route("/shared_state", get(handler_state).post(handler_state_post))
        .with_state(shared_state);

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "7878".to_string());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    println!("Server running on http://{}:{}", host, port);

    axum::serve(listener, app).await.unwrap();
}
