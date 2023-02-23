use actix_web::{get, HttpResponse, Responder};
use maud::html;

use crate::{meta, svgs::Svgs, VERSIONS};

#[get("/changelog.robson")]
pub async fn changelog() -> impl Responder {
  let html = html! {
    (meta())
    title { "Change Log" }
    script src="/static/js/changelog.js" defer {}
    link rel="stylesheet" href="/static/css/changelog.css";


    div id="root" {
      header {
        div.lg.animate onclick="window.location.href='/'" {
          (Svgs::Logo)
        }
        h1{ "Change Log"}
      }
      main {
        div.list {
          @for version in VERSIONS {
            div.version {
              span.version_header {
                h3 { (version.ver) }
                span.line {}
                span.extend { "+" }
              }
              div.log.hide {
                h4 { "Added" }
                div.features {
                  @for feature in version.features {
                    div.feature { (feature) }
                  }
                }
                h4 { "Changes" }
                div.changes {
                  @for change in version.changes {
                    div.change { (change) }
                  }
                }
              }
            }
          }
        }
      }
    }

  };
  HttpResponse::Ok().body(html.into_string())
}
