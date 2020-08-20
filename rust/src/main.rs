use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::thread;

async fn greet() -> impl Responder {
    format!("Hello!")
}

#[derive(Deserialize)]
struct Params {
    number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrimesResult {
    primes: Vec<i32>,
}

impl Responder for PrimesResult {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

async fn generate_primes_handler(params: web::Query<Params>) -> impl Responder {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = generate_primes(params.number);
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    received
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/generate_primes", web::get().to(generate_primes_handler))
            .route("/", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

fn generate_primes(limit: i32) -> PrimesResult {
    let mut result = PrimesResult { primes: Vec::new() };
    let mut counter = 0;
    let mut current_value = 0;

    while counter <= limit {
        if is_prime(current_value) {
            result.primes.push(current_value);
        }
        current_value += 1;
        counter += 1;
    }

    result
}

fn is_prime(n: i32) -> bool {
    match n <= 1 {
        true => false,
        false => {
            for v in 2..n {
                if n % v == 0 {
                    return false;
                }
            }

            true
        }
    }
}
