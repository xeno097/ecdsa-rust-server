use std::{collections::HashMap, sync::RwLock};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct Balances {
    // RwLock is needed to safely share the same data between the different App instaces spawned by Actix.
    balances: RwLock<HashMap<String, i128>>,
}

impl Balances {
    pub fn new() -> Self {
        Self {
            balances: RwLock::new(HashMap::new()),
        }
    }
}

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
    let balances = web::Data::new(Balances::new());

    HttpServer::new(move || {
        App::new()
            .app_data(balances.clone())
            .service(get_address_balance)
            .service(send_transaction)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
