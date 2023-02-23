use maud::{html, PreEscaped};

use crate::components::code::code;

pub fn params_section() -> PreEscaped<String> {
  html! {
    section._Params.hidden {
      p {
        "Params are data that you pass for commands, they can originate for bascially anywhere, from the memory,
        from the stack or more commonly, hardcoded"
      }
      (code("robson robson robson\ncomeu 94"))
      p {
        r#"In the example above, the "comeu" keyword is saying that the value is hardcoded number,
        for representing floating points ou signed ints, use a f{number} and i{number}, respectively"#
      }
      p {
        "Now lets dive in a more complex code"
      }
      (code(r#"robson robson robson
      comeu f12.6
      robson robson robson robson robson robson robson robson robson robson
      comeu 0
      robson
      comeu 0
      fudeu 0
      comeu f1.1"#))
      p {
        "In this example we are settings the value 12.6 into the address 0,
        then adding the value 1.1 and value from address 0, for the finale
        printing it to the screen"
      }
      p {
        "Now let's go through this bit by bit"
      }
      p {
        "In the first command we're adding 12.6 to the stack"
      }
      p {
        "In the seccond we're setting setting the top of the stack
        to the address 0"
      }
      p {
        r#"For the third, we see a "fudeu", fudeu basically is equivalent of the value
        from the address specified, in that case address 0.
        In this command we're adding the two last parameters and pushing to stack the result"#
      }

      p {
        "The last command just print the number to the screen"
      }

      //TODO UPDATE THE PENETROU DOCS

      p {
        r#"Worth mention the "chupou" keyword, it is used when you wanna
        use the stack top as a param, it is only valid as "chupou 0""#
      }
      button {
        "Next"
      }
    }
  }
}
