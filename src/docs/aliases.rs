use maud::{html, PreEscaped};

use crate::components::code::code;

pub fn aliases_section() -> PreEscaped<String> {
  html! {
    section._Aliases.hidden {
      h2 { "Aliases" }
      p {
        "A lot of times, you wanna jump around in your program, and using the lines as an argument
        may not be such an effective method, so robson has aliases,
        aliases are names that you choose for the program to remember certain parts of your program
        instead of hardcoding its location"
      }
      p {
        "Example: "
      }
      (code(r#"robson robson robson
      comeu 69
      robson robson robson robson robson robson robson robson robson
      lambeu :alias
      robson robson robson robson robson robson robson
      alias:
      robson robson robson robson robson robson robson robson"#))
      p {
        "In this example in the seccond command, we jump to the position of the alias,
        completely skiping the third command. this could easly be solved without an alias with"
      }
      (code(r#"robson robson robson
      comeu 69
      robson robson robson robson robson robson robson robson robson
      comeu 4
      robson robson robson robson robson robson robson
      robson robson robson robson robson robson robson robson"#))
      p {
        "On the surface this seems better code, as it has less lines,
        but this code is not maintainable, couse if you for some reason
        add more lines of code before that, the code is gonna be on undefined behavior
        since we dont know what could be at command 6"
      }
      p {
        "This is the end of the guide on Robson, if you wanna an overview of the
        language commands continue"
      }
      button {
        "Next"
      }
    }
  }
}
