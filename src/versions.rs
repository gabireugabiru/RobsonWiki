use actix_web::{get, HttpResponse, Responder};
use maud::html;

use crate::{meta, VERSIONS};

#[get("/versions.robson")]
pub async fn versions() -> impl Responder {
  let html = html! {
    (meta())
    link rel="stylesheet" href="/static/css/versions.css";

    title { "Versions" }
    div id="root" {
      a href="/changelog.robson" {
        "Changelog"
      }
      ul.list {
        @for version in VERSIONS {
          li {
            h3 { (version.ver) }
            a href=(version.get_windows_path()) {
              "Window"
            }
            a href=(version.get_linux_path()) {
              "Linux"
            }
          }
        }
      }
    }
  };
  HttpResponse::Ok().body(html.into_string())
}
