use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("templates/index.html"))
}
#[get("/guide")]
async fn guide() -> impl Responder {
   HttpResponse::Ok().body(include_str!("templates/guide.html")) 
}
#[get("/docs")]
async fn docs() -> impl Responder {
    HttpResponse::Ok().body(include_str!("templates/docs.html"))
}
#[get("/versions")]
async fn versions() -> impl Responder {
    HttpResponse::Ok().body(include_str!("templates/versions.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = match std::env::var("PORT") {
        Ok(a) => a,
        Err(_) => "8080".to_owned()
    };
    
    HttpServer::new(|| {
        App::new()
        .service(actix_files::Files::new("/static", match std::env::var("PORT") {
            Ok(_) => "/app/static",
            Err(_) => "./static"
        }) )
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(index)
            .service(guide)
            .service(docs)
            .service(versions)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
