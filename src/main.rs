
use lockserver::{LockManager, LockError};
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::sync::Arc;
use std::thread;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use std::sync::Mutex as StdMutex;

fn handle_client(stream: TcpStream, manager: Arc<LockManager>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream;
    let mut line = String::new();
    loop {
        line.clear();
        let bytes = reader.read_line(&mut line).unwrap();
        if bytes == 0 { break; }
        let parts: Vec<_> = line.trim().split_whitespace().collect();
        if parts.len() < 3 {
            writeln!(writer, "ERR Invalid command").unwrap();
            continue;
        }
        let cmd = parts[0].to_uppercase();
        let resource = parts[1];
        let owner = parts[2];
        let result = match cmd.as_str() {
            "ACQUIRE" => manager.acquire(resource, owner),
            "RELEASE" => manager.release(resource, owner),
            _ => {
                writeln!(writer, "ERR Unknown command").unwrap();
                continue;
            }
        };
        match result {
            Ok(()) => writeln!(writer, "OK").unwrap(),
            Err(e) => writeln!(writer, "ERR {}", e).unwrap(),
        }
    }
}

#[derive(Deserialize)]
struct LockRequest {
    resource: String,
    owner: String,
}

async fn acquire_lock(
    data: web::Data<Arc<StdMutex<LockManager>>>,
    req: web::Json<LockRequest>,
) -> impl Responder {
    let manager = data.lock().unwrap();
    match manager.acquire(&req.resource, &req.owner) {
        Ok(()) => HttpResponse::Ok().body("OK"),
        Err(e) => HttpResponse::Conflict().body(format!("ERR {}", e)),
    }
}

async fn release_lock(
    data: web::Data<Arc<StdMutex<LockManager>>>,
    req: web::Json<LockRequest>,
) -> impl Responder {
    let manager = data.lock().unwrap();
    match manager.release(&req.resource, &req.owner) {
        Ok(()) => HttpResponse::Ok().body("OK"),
        Err(e) => HttpResponse::Conflict().body(format!("ERR {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = Arc::new(LockManager::new());
    let http_manager = Arc::new(StdMutex::new(LockManager::new()));

    // Start TCP server in a thread
    let tcp_manager = manager.clone();
    thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:4000").expect("Failed to bind port 4000");
        println!("Lockserver TCP listening on 0.0.0.0:4000");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let manager = tcp_manager.clone();
                    thread::spawn(move || handle_client(stream, manager));
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    });

    // Start HTTP server
    println!("Lockserver HTTP listening on 0.0.0.0:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(http_manager.clone()))
            .route("/acquire", web::post().to(acquire_lock))
            .route("/release", web::post().to(release_lock))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
