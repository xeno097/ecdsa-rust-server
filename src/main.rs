use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/balance/{address}")]
async fn get_address_balance() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/send")]
async fn send_transaction() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_address_balance)
            .service(send_transaction)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
