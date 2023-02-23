use maud::{html, PreEscaped};

use crate::svgs::Svgs;

pub fn header() -> PreEscaped<String> {
  let options = [
    ("Docs", "/docs.robson"),
    ("Repository", "https://github.com/gabireugabiru/robson"),
    ("Install", "/guide.robson"),
    ("Try Robson", "/interpreter.robson"),
  ];

  html! {
    header {
      div.animate.lg {
        (Svgs::Logo)
      }

      ul class="navbar" {
        @for (name, hf) in &options {
          li {
            a href=(hf) {
              (name)
            }
          }
        }
        li {
          button id="theme" {
            div;
          }
        }
      }
    }
  }
}
