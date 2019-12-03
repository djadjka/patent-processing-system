use actix_web::{web, App, HttpResponse, HttpServer, Responder};


fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/hello-world", web::get().to(hello_world))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
