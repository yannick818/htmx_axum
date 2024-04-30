use askama_axum::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct MyTemplate {}

#[tokio::main]
async fn main() {
    let app = Router::new()

        .route("/", get(handle_main));

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap()); 
    axum::serve(listener, app).await.unwrap();
}

async fn handle_main() -> impl IntoResponse {
    let template = MyTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}