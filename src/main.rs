use axum::{extract::Path, routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Greeting {
    message: String,
}

async fn greet(Path(name): Path<String>)-> Json<Greeting> {
    let greeting = Greeting {
        message: format!("Hello, {}!", name),
    };
    Json(greeting)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/{name}", get(greet));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Listening on {}!", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
