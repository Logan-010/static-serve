use actix_files as fs;
use actix_web::{web, App, HttpServer, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running server at 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "./").index_file("index.html")) // Serve index.html as the default file
            .route("/hello", web::get().to(|| async {
                HttpResponse::Ok().body("Responded")
            }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
