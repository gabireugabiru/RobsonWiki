use crate::{macros::ele, meta, svgs::Svgs};
use actix_web::{get, HttpResponse, Responder};
use maud::html;
mod aliases;
mod basic;
mod cli;
mod opcodes;
mod overview;
mod params;
#[get("/docs.robson")]
async fn docs() -> impl Responder {
  let navbar_items = [
    ele!("Cli"),
    ele!("Basic Concepts", "Basic"),
    ele!("Opcodes"),
    ele!("Params"),
    ele!("Aliases"),
    ele!("Overview"),
  ];

  let html = html! {
    (meta())
    link rel="stylesheet" href="/static/css/docs.css";
    script src="/static/js/docs.js" defer {}

    div id="root" {
      navbar {
        button.hidden {
          "|||"
        }
        ul id="list" {
          @for (name, id) in navbar_items {
            li id=(id) {
              (name)
            }
          }
        }
        div.lg.animate id="logo" {
          (Svgs::Logo)
        }
      }

      main {
        button.hidden id="outside" {
          "|||"
        }
        (cli::cli_section())
        (basic::basic_section())
        (opcodes::opcodes_section())
        (params::params_section())
        (aliases::aliases_section())
        (overview::overview_section())
      }
    }
  };

  HttpResponse::Ok().body(html.into_string())
}
