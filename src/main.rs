use dotenvy::dotenv;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use lockserver::LockManager;
use serde::Deserialize;
use std::env;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
//
use clap::{Arg, Command};

#[derive(Deserialize)]
struct LockRequest {
    resource: String,
    owner: String,
}

fn check_secret(req: &HttpRequest, expected: &str) -> bool {
    req.headers()
        .get("X-LOCKSERVER-SECRET")
        .map(|v| v == expected)
        .unwrap_or(false)
}

async fn acquire_lock(
    data: web::Data<Arc<StdMutex<LockManager>>>,
    req: web::Json<LockRequest>,
    http_req: HttpRequest,
    secret: web::Data<String>,
) -> impl Responder {
    if !check_secret(&http_req, &secret) {
        return HttpResponse::Unauthorized().body("Missing or invalid secret");
    }
    let manager = data.lock().unwrap();
    match manager.acquire(&req.resource, &req.owner) {
        Ok(()) => HttpResponse::Ok().body("OK"),
        Err(e) => HttpResponse::Conflict().body(format!("ERR {}", e)),
    }
}

async fn release_lock(
    data: web::Data<Arc<StdMutex<LockManager>>>,
    req: web::Json<LockRequest>,
    http_req: HttpRequest,
    secret: web::Data<String>,
) -> impl Responder {
    if !check_secret(&http_req, &secret) {
        return HttpResponse::Unauthorized().body("Missing or invalid secret");
    }
    let manager = data.lock().unwrap();
    match manager.release(&req.resource, &req.owner) {
        Ok(()) => HttpResponse::Ok().body("OK"),
        Err(e) => HttpResponse::Conflict().body(format!("ERR {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file if present
    let _ = dotenv();
    // Use clap for --argument style parsing
    let matches = Command::new("lockserver")
        .about("Distributed lock server for coordinating access to shared resources.")
        .arg(
            Arg::new("bind")
                .long("bind")
                .short('b')
                .value_name("BIND_IP")
                .help("Bind IP address (default: 0.0.0.0)"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .value_name("PORT")
                .help("HTTP API port (default: 8080)"),
        )
        .get_matches();

    // Load from env first, then override with CLI args if present
    let mut bind_ip = env::var("LOCKSERVER_BIND_IP").unwrap_or_else(|_| "0.0.0.0".to_string());
    let mut http_port = env::var("LOCKSERVER_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    let secret = env::var("LOCKSERVER_SECRET").unwrap_or_else(|_| "changeme".to_string());

    if let Some(cli_bind) = matches.get_one::<String>("bind") {
        bind_ip = cli_bind.clone();
    }
    if let Some(cli_port) = matches.get_one::<String>("port")
        && let Ok(port) = cli_port.parse()
    {
        http_port = port;
    }
    // Optionally allow CLI arg for secret in future

    let http_manager = Arc::new(StdMutex::new(LockManager::new()));
    let http_addr = (bind_ip.as_str(), http_port);
    println!(
        "Lockserver HTTP listening on {}:{} (secret required)",
        bind_ip, http_port
    );
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(http_manager.clone()))
            .app_data(web::Data::new(secret.clone()))
            .route("/acquire", web::post().to(acquire_lock))
            .route("/release", web::post().to(release_lock))
    })
    .bind(http_addr)?
    .run()
    .await
}
