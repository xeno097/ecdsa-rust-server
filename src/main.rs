use std::{collections::HashMap, sync::RwLock};

use actix_web::{
    body::BoxBody, get, http::header::ContentType, post, web, App, HttpResponse, HttpServer,
    Responder,
};
use serde::Serialize;

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

#[derive(Serialize)]
struct BalanceResponse {
    balance: i128,
}

impl Responder for BalanceResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        match serde_json::to_string(&self) {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[get("/balance/{address}")]
async fn get_address_balance(
    data: web::Data<Balances>,
    address: web::Path<String>,
) -> impl Responder {
    let address = address.into_inner();

    match data.balances.read().unwrap().get(&address) {
        Some(balance) => BalanceResponse { balance: *balance },
        None => BalanceResponse { balance: 0 },
    }
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
