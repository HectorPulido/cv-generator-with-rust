mod generator;
use actix_web::dev::BodyEncoding;
use actix_web::http::ContentEncoding;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use generator::Generator;
use std::env;
use std::fs::File;
use std::io::Read;

#[post("/generate_cv")]
async fn generate_cv(info: web::Json<serde_json::Value>) -> impl Responder {
    let mut generator = Generator::new();
    let markdown = generator.format(info.into_inner());
    let html = Generator::generate_html(markdown);
    let final_html = Generator::apply_theme(html);
    Generator::generate_pdf(final_html);

    // File content may come from a database as a blob data
    let mut f = File::open("basic.pdf").unwrap();
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).expect("File failed to read");

    HttpResponse::Ok()
        .encoding(ContentEncoding::Identity)
        .content_type("application/pdf")
        .header("accept-ranges", "bytes")
        .header(
            "content-disposition",
            "attachment; filename=\"download-angular.pdf\"",
        )
        .body(buffer)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");

    println!("Running http://{}:{}", host, port);

    HttpServer::new(|| App::new().service(generate_cv))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
