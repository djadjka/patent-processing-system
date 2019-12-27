#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
#[macro_use] 
extern crate failure;

mod models;
mod routes;
mod services;

use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let session = services::scylla::create_session();
    HttpServer::new(move || {
        App::new()
            .data(models::State {
                session: session.clone(),
            })
            .configure(routes::concated_resources)
    })
    .keep_alive(None)
    .bind("127.0.0.1:8088")?
    .start()
    .await
}
