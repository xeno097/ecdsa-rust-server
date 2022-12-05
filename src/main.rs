use std::{collections::HashMap, sync::RwLock};

use actix_cors::Cors;
use actix_web::{
    body::BoxBody, get, http::header::ContentType, post, web, App, HttpResponse, HttpServer,
    Responder,
};
use ecdsa_rust_server::eth::{get_address, recover_key};
use serde::{Deserialize, Serialize};

// Ethereum accounts from Hardhat
// Account #0: 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266
// Private Key: 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

// Account #1: 0x70997970c51812dc3a010c7d01b50e0d17dc79c8
// Private Key: 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d

// Account #2: 0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc
// Private Key: 0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a

struct Balances {
    // RwLock is needed to safely share the same data between the different App instaces spawned by Actix.
    balances: RwLock<HashMap<String, i128>>,
}

impl Balances {
    pub fn new() -> Self {
        let mut balances: HashMap<String, i128> = HashMap::new();

        balances.insert(
            String::from("f39fd6e51aad88f6f4ce6ab8827279cfffb92266"),
            150,
        );
        balances.insert(
            String::from("70997970c51812dc3a010c7d01b50e0d17dc79c8"),
            150,
        );
        balances.insert(
            String::from("3c44cdddb6a900fa2b585dd299e03d12fa4293bc"),
            150,
        );

        Self {
            balances: RwLock::new(balances),
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

#[derive(Deserialize, Debug)]
struct Transaction {
    amount: i128,
    to: String,
    s: String,
    r: u8,
}

#[derive(Serialize)]
struct TransactionData {
    amount: i128,
    to: String,
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

// TODO: add better error management.
#[post("/send")]
async fn send_transaction(
    data: web::Data<Balances>,
    transaction: web::Json<Transaction>,
) -> impl Responder {
    let transaction_data = TransactionData {
        amount: transaction.amount,
        to: transaction.to.clone(),
    };

    let raw_data = serde_json::to_string(&transaction_data).unwrap();

    let sender_pub_key = recover_key(raw_data.as_bytes(), &transaction.s, transaction.r).unwrap();

    let sender_address = get_address(&sender_pub_key).unwrap();

    let mut balances = data.balances.write().unwrap();

    let sender_balance = balances.entry(sender_address.clone()).or_insert(0);

    if *sender_balance < transaction.amount {
        return BalanceResponse { balance: 0 };
    }

    *sender_balance -= transaction.amount;

    let receiver_balancer = balances.entry(transaction.to.clone()).or_insert(0);
    *receiver_balancer += transaction.amount;

    // It is safe to call unwrap because the key exists now.
    let sender_balance = balances.get(&sender_address).unwrap();

    BalanceResponse {
        balance: *sender_balance,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let balances = web::Data::new(Balances::new());

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(balances.clone())
            .wrap(cors)
            .service(get_address_balance)
            .service(send_transaction)
    })
    .bind(("127.0.0.1", 3042))?
    .run()
    .await
}
