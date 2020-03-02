extern crate html_diff;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

async fn health_check() -> impl Responder {
    "ok"
}
#[derive(Deserialize)]
struct Diff {
    current: String,
    old: String,
}
#[derive(Serialize, Deserialize)]
struct DiffResponse {
    result: String,
}

async fn diff(form: web::Json<Diff>) -> Result<HttpResponse> {
    let diff = html_diff::diff(&form.old, &form.current);

    Ok(HttpResponse::Ok().json(DiffResponse { result: diff }))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/diff", web::post().to(diff))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
