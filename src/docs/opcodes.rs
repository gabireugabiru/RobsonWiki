use maud::{html, PreEscaped};

use crate::components::code::code;

pub fn opcodes_section() -> PreEscaped<String> {
  fn list(list: &[&str]) -> PreEscaped<String> {
    html! {
      @for item in list {
        li { (item) }
      }
    }
  }
  html! {
    section._Opcodes.hidden {
      p {
        "The opcodes in robson are the numbers wich represents functions within the language,
        opcodes are represented as the number of robson in one line."
      }
      (code("robson robson robson"))
      h2 {
        "List of opcodes"
      }
      ol {
        (list(&[
          "Operations like sum and subtraction",
          "If lower jump",
          "Push to stack",
          "If equal jump",
          "Verify stack",
          "User input",
          "Print ascci",
          "Print number",
          "Jump",
          "Set to memory",
          "Get string buffer",
          "Time commands",
          "Flush",
          "Random float"
        ]))
      }
      p {
        "Every opcode has a different number of params varying between 0 and 3"
      }
      button { 
        "Next"
      }
    }
  }
}
