use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("templates/index.html"))
}

#[get("/guide")]
async fn guide() -> impl Responder {
   HttpResponse::Ok().body(include_str!("templates/guide.html")) 
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap();
    
    HttpServer::new(|| {
        App::new()
        .service(actix_files::Files::new("/static", "/app/static") )
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(index)
            .service(guide)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
