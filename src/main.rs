use std::str::FromStr;

use actix_web::{get, App, HttpServer, Responder, HttpResponse, HttpRequest, dev::Service, http::header::{HeaderName, HeaderValue}};



use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;


pub struct NoCache;

impl<S, B> Transform<S, ServiceRequest> for NoCache
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = NoCacheMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(NoCacheMiddleware { service }))
    }
}

pub struct NoCacheMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for NoCacheMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            let headers = res.headers_mut();
            let name = HeaderName::from_str("Cache-Control");
            let value = HeaderValue::from_str("no-cache");

            if let Ok(name) = name {
                if let Ok(value) = value {
                    headers.append(name, value);
                }
            }
            Ok(res)
        })
    }
}


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
#[get("/interpreter")]
async fn interpreter() -> impl Responder {
    HttpResponse::Ok().body(include_str!("templates/interpreter.html"))
}

#[get("/dynamic/css/vars.css")]
async fn vars(req: HttpRequest) -> impl Responder {
    
    let cookie_theme = req.cookie("theme");
    if let Some(cookie_theme) = &cookie_theme {
        let value = cookie_theme.value();
        if value == "dark" {
            return HttpResponse::Ok().body(include_str!("css/vars.dark.css"));
        }
    }
    HttpResponse::Ok().body(include_str!("css/vars.css"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = match std::env::var("PORT") {
        Ok(a) => a,
        Err(_) => "8080".to_owned()
    };
    HttpServer::new(|| {
        let file = match std::env::var("PORT") {
            Ok(_) => "/app/static",
            Err(_) => "./static"
        };
        let a = App::new()
            .service(actix_files::Files::new("/static", file ))
            .service(index)
            .service(guide)
            .service(docs)
            .service(versions)
            .service(interpreter)
            .service(vars);

        
        #[cfg(not(debug_assertions))]
        return a;
        #[cfg(debug_assertions)]
        a.wrap(NoCache)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
