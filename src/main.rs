use axum::{ routing::get, response::{self, IntoResponse}, Router};
use async_graphql::{http::GraphiQLSource, Object, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use tokio::{net::TcpListener, signal};

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

    axum::serve(listener, app.into_make_service())
          .with_graceful_shutdown(
              async {
            let ctrl_c = async {
                signal::ctrl_c()
                    .await
                    .expect("failed to install Ctrl+C handler");
                println!("Ctrl+C received, shutting down");
            };

            let terminate = async {
                signal::unix::signal(signal::unix::SignalKind::terminate())
                    .expect("failed to install SIGTERM handler")
                    .recv()
                    .await;
                println!("SIGTERM received, shutting down");
            };

            tokio::select! {
                _ = ctrl_c => {},
                _ = terminate => {},
            }

            println!("signal received, starting graceful shutdown");
        }
          )
        .await.unwrap();
}
