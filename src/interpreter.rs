use actix_web::{get, HttpResponse, Responder};
use maud::html;

use crate::{meta, svgs::Svgs};

#[get("/interpreter.robson")]
async fn interpreter() -> impl Responder {
  let html = html! {
    (meta())
    link rel="stylesheet" href="/static/css/interpreter.css";

    title {
      "Mãe do Ranan"
    }
    div id = "root" {
      header {
        div.lg.animate id="logo" {
          (Svgs::Logo)
        }
        span {
          "This robson version is highly outdated"
        }
        a href="/docs.robson" {
          "Learn robson here"
        }
      }
      div id="syntax" {
        div id="formated" {

        }
        div contenteditable spellcheck="false" id="code" {}
      }
      div.row {
        div {
          button id="run" {
            "Run"
          }
          div.finished id="status" {
            "Finished"
          }
        }
        div {
          button id="reset" {
            "Reset"
          }
          input type="text" id="stdin";
          button id="enter" {
            "Enter"
          }
        }
      }
      div id="output" {

      }
      script type="module" src="/static/js/interpreter.js" {}
    }

  };
  HttpResponse::Ok().body(html.into_string())
}
