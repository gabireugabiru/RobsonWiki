use actix_web::{get, HttpResponse, Responder};
use maud::html;

use crate::{components::header::header, meta, svgs::Svgs};

#[get("/")]
pub async fn index() -> impl Responder {
  let html = html! {
    (meta())
    title { "Robson" }
    link rel="stylesheet" href="/static/css/index.css";
    script src="/static/js/index.js" defer {}

    div id="root" {

      (header())
      main {
        div.presentation {
          h1 {
            "Robson"
            span {
              "0.1.6"
            }
          }
          p {
            "Robson is a vision of the future of the verbose driven instant legacy code
            style. It has heavy inspirations in java, by repeating the same thing over
            and over, but has also the procedural nature of assembly and HolyC"
          }
        }
        div.c {
            (Svgs::Curves)
        }
        div.why {
          h2 { "Why Robson?" }
          div {
            section {
              h3 { "Advantages" }
              div.advantages {
                "As today it comes to solve to problem of doing things too fast,
                as to solve it robson creates a lot of redundant and repetetive code
                wich may seem bad at first but if you think about it, more time you
                spend coding more money you will make (in case of hourly payments)"
              }
            }
            section {
              h3 { "Where to use" }
              div.shines {
                "robson is made to be used in cli application with focus on
                control of input, where as the security of the robsons runtime
                prevent as much as all io errors"
              }
            }
            section {
              h3 { "Future" }
              div.future {
                "Robson is a really new programming language and has a lot of to
                go for, but its growing fast, with features coming frequently, feel
                free to contribute"
                a.highlight target="_blank" href="https://github.com/gabireugabiru/robson" {
                  "here"
                }
              }
            }
          }
        }
        div.c.reverse {
            (Svgs::Curves)
        }
      }
      footer {
        div {
          "Thanks for supporting the growing robson ecosystem, hope you had a good time with us"
          div.t {
            (Svgs::Hand)
          }
        }

        div.robson {
          "A special thanks to Robson"
          img.robson src="/static/robson_real.jpeg";
        }
      }
    }
  };
  HttpResponse::Ok().body(html.into_string())
}
