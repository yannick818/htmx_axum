use std::sync::{atomic::AtomicUsize, Arc};

use askama_axum::Template;
use axum::{
    response::IntoResponse, routing::{get, post}, Router
};

#[derive(Template)]
#[template(path = "index.html", block = "index")]
pub struct IndexHtml {
    count: usize,
}

#[derive(Template)]
#[template(path = "index.html", block = "counter")]
pub struct CounterHtml {
    count: usize,
}

#[tokio::main]
async fn main() -> ! {
    let count = Arc::new(AtomicUsize::new(0));

    let app = Router::new()
        .route("/", get({
            let count = count.clone();
            move || get_counter(count) 
            }
        ))
        .route("/count", post({
            let count = count.clone();
            move || inc_counter(count)
            }
        ));

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap()); 
    axum::serve(listener, app).await.unwrap();
    panic!("the server has stopped");
}

async fn inc_counter(count: Arc<AtomicUsize>) -> impl IntoResponse {
    count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    CounterHtml{count: count.load(std::sync::atomic::Ordering::Relaxed)}
}

async fn get_counter(count: Arc<AtomicUsize>) -> impl IntoResponse {
    let count = count.load(std::sync::atomic::Ordering::Relaxed);
    IndexHtml{count}
}