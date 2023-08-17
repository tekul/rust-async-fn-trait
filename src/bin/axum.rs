#![feature(async_fn_in_trait, return_type_notation)]
use std::net::SocketAddr;

use axum::{extract::State, response::IntoResponse, routing::get, Router};

pub struct Data {
    id: String,
}

pub trait Database {
    async fn load_data(&self, id: &str) -> Data;
}

#[derive(Clone)]
struct SillyDatabase {}

impl Database for SillyDatabase {
    async fn load_data(&self, id: &str) -> Data {
        Data { id: id.to_string() }
    }
}

pub fn mk_app<B>(backend: B) -> Router
where
    B: Clone + Send + Sync + Database<load_data(): Send> + 'static,
{
    Router::new()
        .route("/data", get(get_data::<B>))
        .with_state(backend)
}

async fn get_data<B>(State(backend): State<B>) -> impl IntoResponse
where
    B: Database,
{
    let Data { id } = backend.load_data("some_id").await;
    format!("Loaded data, with id {id}")
}

#[tokio::main]
async fn main() {
    let backend = SillyDatabase {};
    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 8088));
    println!("Starting server on {addr}");
    axum_server::bind(addr)
        .serve(mk_app::<SillyDatabase>(backend).into_make_service())
        .await
        .expect("server error");
}
