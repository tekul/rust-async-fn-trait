use actix_web::{
    web::{self, ServiceConfig},
    App, HttpRequest, HttpResponse, HttpServer,
};
use async_trait::async_trait;

pub struct Data {
    id: String,
}

#[async_trait]
pub trait Database {
    async fn load_data(&self, id: &str) -> Data;
}

#[derive(Clone)]
struct SillyDatabase {}

#[async_trait]
impl Database for SillyDatabase {
    async fn load_data(&self, id: &str) -> Data {
        Data { id: id.to_string() }
    }
}

pub fn mk_app<B>(backend: B) -> impl FnOnce(&mut ServiceConfig)
where
    B: Database + 'static,
{
    move |app| {
        app.app_data(backend)
            .service(web::resource("/data").to(get_data::<B>));
    }
}

async fn get_data<B>(req: HttpRequest) -> HttpResponse
where
    B: Database + 'static,
{
    let backend = req
        .app_data::<B>()
        .expect("app_data should include Database");
    let Data { id } = backend.load_data("some_id").await;
    HttpResponse::Ok().body(format!("Loaded data, with id '{id}'"))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database = SillyDatabase {};
    let server = HttpServer::new(move || App::new().configure(mk_app(database.clone())))
        .bind("127.0.0.1:8088")
        .unwrap();
    println!("Starting app on port 8088");
    server.run().await
}
