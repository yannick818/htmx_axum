use std::sync::Arc;

use askama_axum::{IntoResponse, Template};
use axum::{
    routing::get,
    Router,
};
use tokio::sync::Mutex;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexHtml {
    count: usize,
}

#[tokio::main]
async fn main() -> ! {
    let count = Arc::new(Mutex::new(0usize));

    let counter_handler = {
        move || {
            async move {
                let mut count = count.lock().await;
                *count += 1;
                IndexHtml{count: *count}
            }
        }
    };

    let app = Router::new().route("/", get(
        counter_handler
    ));

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap()); 
    axum::serve(listener, app).await.unwrap();
    panic!("the server has stopped");
}
