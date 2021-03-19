use rustls::internal::pemfile::pkcs8_private_keys;
use rustls::internal::pemfile::certs;
use std::io::BufReader;
use std::fs::File;
use actix_web::{
    web,
    App,
    HttpServer,
    Responder,
    HttpResponse,
    middleware};

use serde::{
    Serialize,
    Deserialize
};

use std::env;

use rustls::{NoClientAuth, ServerConfig};

const PORT: i32 = 3002;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Value {
    pub value: String,
}

async fn text() -> impl Responder {
    "value"
}

async fn json() -> impl Responder {
    HttpResponse::Ok().json(Value {
        value: String::from("value")
    })
}

async fn body(data: web::Json<serde_json::Value>) -> impl Responder {
    let parsed: serde_json::Value = serde_json::from_value(data.0).unwrap();
    HttpResponse::Ok().json(parsed)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on port {}", PORT);
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new())
            .app_data(web::PayloadConfig::new(1000000 * 250)) // 6 megs max
            .route("/json", web::get().to(json))
            .route("/text", web::get().to(text))
            .route("/body", web::post().to(body))
        });

    if env::var("USE_TLS").unwrap_or_default() != "" {
        let mut config = ServerConfig::new(NoClientAuth::new());
        let cert_file = &mut BufReader::new(File::open("../certs/cert.crt").unwrap());
        let key_file = &mut BufReader::new(File::open("../certs/key.key").unwrap());
        let cert_chain = certs(cert_file).unwrap();
        let mut keys = pkcs8_private_keys(key_file).unwrap();
        config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

        server.bind_rustls(format!("127.0.0.1:{}", PORT), config)?
            .run()
            .await
    } else {
        server.bind(format!("127.0.0.1:{}", PORT))?
        .run()
        .await
    }   
}