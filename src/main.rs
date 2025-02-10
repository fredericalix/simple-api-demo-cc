use actix_web::{web, App, HttpResponse, HttpServer};
use serde_json::json;
use std::env;

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

// Routes for the second server
async fn root() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "status": "ok" }))
}

async fn public_route() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "message": "public route" }))
}

async fn private_route() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "message": "private and protected route" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // First server configuration
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port.parse::<u16>().expect("PORT must be a valid number");

    // Second server configuration
    let port_app = env::var("PORT_APP").unwrap_or_else(|_| "4242".to_string());
    let port_app = port_app.parse::<u16>().expect("PORT_APP must be a valid number");

    println!("Main server started on port {}", port);
    println!("Application server started on port {}", port_app);

    // Launch both servers in parallel
    let server1 = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
    .bind(("0.0.0.0", port))?;

    let server2 = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(root))
            .route("/public", web::get().to(public_route))
            .route("/private", web::get().to(private_route))
    })
    .bind(("0.0.0.0", port_app))?;

    // Start both servers
    futures::future::try_join(server1.run(), server2.run()).await?;

    Ok(())
}
