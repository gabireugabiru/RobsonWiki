use std::str::FromStr;

use actix_web::{
  dev::Service,
  get,
  http::header::{HeaderName, HeaderValue},
  App, HttpRequest, HttpResponse, HttpServer, Responder,
};

use maud::{html, PreEscaped, DOCTYPE};

use std::future::{ready, Ready};

use actix_web::{
  dev::{forward_ready, ServiceRequest, ServiceResponse, Transform},
  Error,
};
use futures_util::future::LocalBoxFuture;

mod changelog;
mod docs;
mod guide;
mod index;
mod interpreter;
mod macros;
mod svgs;
mod versions;
mod components {
  pub mod code;
  pub mod header;
  pub mod memory;
}

pub struct Version {
  ver: &'static str,
  changes: &'static [&'static str],
  features: &'static [&'static str],
}
impl Version {
  pub const fn new(
    ver: &'static str,
    changes: &'static [&'static str],
    features: &'static [&'static str],
  ) -> Self {
    Self {
      changes,
      ver,
      features,
    }
  }
  pub fn get_windows_path(&self) -> String {
    format!("/static/versions/{}vWindows.zip", self.ver)
  }
  pub fn get_linux_path(&self) -> String {
    format!("/static/versions/{}vLinux.zip", self.ver)
  }
}

const VERSIONS: &[Version] = &[
  Version::new(
    "0.1.6",
    &[
      "Added robson macros",
      "Added param declaration in robson macros",
      "Added text expression for macro params",
      "Added terminal command for changing background color",
    ],
    &[],
  ),
  Version::new(
    "0.1.5",
    &["Fixed the multiple lambeu glitch in push opcodes"],
    &["Added function to change color of text in terminal commands"],
  ),
  Version::new(
    "0.1.4",
    &["Now error of invalid flags to show all valid flags"],
    &[
      "Added --chars for getting ascci values",
      "Added opcode 16 for random numbers",
      "Added type conversion",
    ],
  ),
  Version::new(
    "0.1.3",
    &["Now --version shows robson's logo"],
    &[
      "Added --generate for getting string raw values",
      "Added robsons keyword for including other robson files in the robson binary",
    ],
  ),
  Version::new(
    "0.1.2",
    &[
      "Fixed path lowercasing",
      "Increased cohesion in terminal commands",
      "Now compiled robsons are placed in ./out",
      "Terminal operations now are queued, so they will only run when flushed",
      "Removed some abstractions in terminal commands",
    ],
    &[],
  ),
  Version::new(
    "0.1.1",
    &[],
    &[
      "Opcode 13 for time operations",
      "Opcode 14 for flushing terminal",
      "Opcode 15 for raw terminal manipulation",
    ],
  ),
  Version::new(
    "0.1.0",
    &[
      "Changed from full interpreted, to an compiled/interpreted",
      "Up to 300% of performace improvement",
      "Now for run an .robson you need to use the run flag, or just compile it and run the rbsn",
    ],
    &[
      "An unique binary type for robson, rbsn",
      "Time flag to know runtime of a rbsn",
    ],
  ),
  Version::new(
    "0.0.9",
    &[
      "Fixed pop function",
      "Fixed lambeu keyword not working on abreviated pushs",
    ],
    &["Added penetrou keyword for double pointer"],
  ),
  Version::new(
    "0.0.8",
    &[
      "Changed opcode 2 from substracion to an if_lower statement",
      "Opcode 1 now has all operations supported",
    ],
    &[],
  ),
  Version::new(
    "0.0.7",
    &["Fixed bug where you couldn't add two floating points or signed ints"],
    &[],
  ),
  Version::new(
    "0.0.6",
    &[
      "Now robson store 32bit data with type safety",
      "Changed the way robson print color to be compatible with all terminals",
    ],
    &["Added syntax for storing floating points and signed integers"],
  ),
  Version::new(
    "0.0.5",
    &["Now robson's cli prints colorful outputs"],
    &["Added jump aliases"],
  ),
];

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

fn meta() -> PreEscaped<String> {
  html! {
    (DOCTYPE)
    meta charset="UTF-8";
    meta http-equiv="X-UA-Compatible" content="IE=edge";
    meta name="viewport" content="width=device-width, initial-scale=1.0";
    link rel="stylesheet" href="/dynamic/css/vars.css";
    link rel="stylesheet" href="/static/css/logo.css";

    link rel="icon" type="image/svg+xml" href="/static/favicon.svg";
    link rel="icon" type="image/png" href="/static/favicon.png";
    script src="/static/js/logo.js" defer {}
  }
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
    Err(_) => "5150".to_owned(),
  };
  HttpServer::new(|| {
    let file = match std::env::var("PORT") {
      Ok(_) => "/app/static",
      Err(_) => "./static",
    };
    let a = App::new()
      .service(actix_files::Files::new("/static", file))
      .service(index::index)
      .service(guide::guide)
      .service(docs::docs)
      .service(versions::versions)
      .service(interpreter::interpreter)
      .service(vars)
      .service(changelog::changelog);

    #[cfg(not(debug_assertions))]
    return a;
    #[cfg(debug_assertions)]
    a.wrap(NoCache)
  })
  .bind(format!("0.0.0.0:{}", port))?
  .run()
  .await
}
