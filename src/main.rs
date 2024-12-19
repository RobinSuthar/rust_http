use axum::{Router, routing::get};
use std::net::SocketAddr;


async fn hello_world() -> &'static str {
    "Hello world!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();


}