use axum::{ routing::get, response::{self, IntoResponse}, Router};
use async_graphql::{http::GraphiQLSource, Object, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use tokio::net::TcpListener;

struct Query;

#[Object]
impl Query {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
let schema= Schema::new(Query, EmptyMutation, EmptySubscription);

    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Listening on {}!", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
