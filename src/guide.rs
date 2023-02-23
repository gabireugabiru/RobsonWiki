use actix_web::{get, HttpResponse, Responder};
use maud::html;

use crate::{components::code::code, meta, svgs::Svgs, VERSIONS};

#[get("/guide.robson")]
async fn guide() -> impl Responder {
  let html = html! {
    (meta())

    link rel="stylesheet" href="/static/css/guide.css";
    script src="/static/js/guide.js" defer {}

    title {
      "Guide"
    }
    div.lg animate id="logo" {
      (Svgs::Logo)
    }
    navbar {
      ul id="list" {
        li id="Download" {
          "Download"
        }
        li id="Setting" {
          "Setting up"
        }
        li id ="Getting" {
          "Getting Started"
        }
      }
    }
    main {
      section._Download {
        h2 {
          "Download"
        }
        p {
          "Download based on your operating system"
        }
        a.download download="windows.zip" href=(VERSIONS[0].get_windows_path()) {
          "Windows executable"
        }
        a.download download="linux.zip" href=(VERSIONS[0].get_linux_path()) {
          "Linux binary"
        }
        a.download href="/versions.robson" {
          "Other versions"
        }
        p {
          "After download extract the zip"
        }
      }
      section._Setting {
        h2 {
          "Setting up"
        }
        h3 {
          "Windows"
        }
        h3 {
          "Easy way, needs restart"
        }
        p {
          "Run install_robson"
        }
        h3 {
          "Hard way, doesn't need restart"
        }
        p {
          r#"First hit windows key and search "edit environment variables""#
        }
        p {
          "Then click edit on path variable"
        }
        img src="/static/part1.jpeg";
        p {
          "After that click new and add the path to the robson.exe"
        }
        p {
          "If something wasn't clear, try searching for environment variables in windows"
        }
        h3 {
          "Linux/Wsl"
        }
        p {
          "Run the command below inside the place you downloaded the file, it will copy the robson binary and put it in the binaries folder"
        }
        (code("# sudo cp robson /usr/bin"))
        h3 {
          "Mac os"
        }
        p {
          "Cry"
        }
        h3 {
          "Test the installation"
        }
        p {
          "Run the following in your terminal"
        }
        (code("$ robson --version"))
        p {
          "If you get an error, well, something went wrong"
        }
      }
      section._Getting {
        h2 {
          "Getting start"
        }
        p {
          "For you to start learning robson TODAY, we highly reccomend checking our docs"
        }
        a.end href="/docs.robson" {
          "LEARN ROBSON NOW"
        }
      }
    }
  };

  HttpResponse::Ok().body(html.into_string())
}
