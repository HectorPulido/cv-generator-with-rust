mod generator;

use generator::Generator;

use actix_web::dev::BodyEncoding;
use actix_web::http::ContentEncoding;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use rand::Rng;
use std::env;
use std::path::Path;
use std::fs::{remove_file, File};
use std::io::{Read, BufReader};

fn create_pdf(json_data: serde_json::Value) -> String {
    let mut rng = rand::thread_rng();
    let random_name = format!("cv_{}.pdf", rng.gen_range(0..99999));
    let mut generator = Generator::new();
    let html = generator.format(json_data);
    let html = generator.apply_theme(html);
    Generator::generate_pdf(html, &random_name);

    return random_name;
}


#[post("/generate_cv")]
async fn generate_cv(info: web::Json<serde_json::Value>) -> impl Responder {
    let json_data = info.into_inner();
    
    let random_name = create_pdf(json_data);

    // File content may come from a database as a blob data
    let mut f = File::open(&random_name).expect("Failed to read file");
    let mut buffer = Vec::new();

    // Read the whole file
    f.read_to_end(&mut buffer).expect("File failed to read");

    // Delete file
    remove_file(&random_name).expect("Failed to delete file");

    let filename = format!("attachment; filename=\"{}\"", random_name);

    HttpResponse::Ok()
        .encoding(ContentEncoding::Identity)
        .content_type("application/pdf")
        .header("accept-ranges", "bytes")
        .header("content-disposition", filename)
        .body(buffer)
}

fn read_json_from_file<P: AsRef<Path>>(path: P) -> serde_json::Value {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    return serde_json::from_reader(reader).unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if let Some(json_path) = args.get(1){
        let json_data: serde_json::Value = read_json_from_file(json_path);
        let random_name = create_pdf(json_data);
        println!("Path of the generated cv: {random_name}");
        return Ok(());
    }

    dotenv().ok();

    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");

    println!("Running http://{}:{}", host, port);

    HttpServer::new(|| App::new().service(generate_cv))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
