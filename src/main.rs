use askama_axum::Template;
use axum::{
    routing::get,
    Router,
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexHtml {}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async {IndexHtml{}}));

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap()); 
    axum::serve(listener, app).await.unwrap();
}
