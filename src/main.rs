extern crate html_diff;
use actix_web::middleware;
use actix_web::{web, App, FromRequest, HttpResponse, HttpServer, Responder, Result};
use askama::Template;
use serde::{Deserialize, Serialize};

extern crate log;

// Health check page to make sure we are up
async fn health_check() -> impl Responder {
    "ok"
}

#[derive(Template)]
#[template(path = "diff_form.html")]
struct DiffFormTemplate {}

// for bugging a simple form view
async fn diff_form() -> Result<HttpResponse> {
    let template = DiffFormTemplate {}.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(template))
}

#[derive(Deserialize)]
struct Diff {
    current: String,
    old: String,
}

#[derive(Template)]
#[template(path = "diff_results.html")]
struct DiffResultsTemplate {
    current: String,
}
// Diffing method via form
async fn diff_results(form: web::Form<Diff>) -> Result<HttpResponse> {
    let diff = html_diff::diff(&form.old, &form.current, None);
    let template = DiffResultsTemplate { current: diff }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(template))
}

#[derive(Serialize, Deserialize)]
struct DiffResponse {
    result: String,
}
// Diffing method via json
async fn diff(form: web::Json<Diff>) -> Result<HttpResponse> {
    let diff = html_diff::diff(&form.old, &form.current, None);
    Ok(HttpResponse::Ok().json(DiffResponse { result: diff }))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .app_data(web::Form::<Diff>::configure(|cfg| cfg.limit(4000097)))
            .app_data(web::Json::<Diff>::configure(|cfg| cfg.limit(40000000097)))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .route("/diff", web::post().to(diff))
            .route("/diff_form", web::get().to(diff_form))
            .route("/diff_form", web::post().to(diff_results))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
