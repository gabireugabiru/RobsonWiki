use maud::{html, PreEscaped};

pub fn code(code: &str) -> PreEscaped<String> {
  let lines = code.lines();
  fn spaced(str: &str) -> Vec<&str> {
    str.split(" ").collect::<Vec<&str>>()
  }

  fn is_literal(str: &str) -> bool {
    let f = str.split("f").collect::<Vec<&str>>();
    let i = str.split("i").collect::<Vec<&str>>();
    if f.len() == 2 && f[1].parse::<f32>().is_ok() {
      return true;
    }
    if i.len() == 2 && i[1].parse::<i32>().is_ok() {
      return true;
    }
    if str.parse::<u32>().is_ok() {
      return true;
    }

    false
  }

  fn is_lambeu(word: &str) -> bool {
    if !word.contains(":") {
      return false;
    }
    let splited = word
      .split(":")
      .filter(|w| !w.is_empty())
      .collect::<Vec<&str>>();
    if splited.len() != 1 {
      return false;
    }
    true
  }

  html! {
    div.code {
      @for line in lines {
        @for word in spaced(line) {
          @match word  {
            "robson" => span.keyword {(word)},
            _ => {
              @if is_literal(word) {
                span.literal { (word) }
              } @else if is_lambeu(word) {
                span.alias {
                  (word)
                }
              } @else {
                (word)
              }
            }
          }
          " "
        }
        br;
      }
    }
  }
}
