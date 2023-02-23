use maud::{html, PreEscaped};

use crate::components::code::code;

pub fn overview_section() -> PreEscaped<String> {
  html! {
    section._Overview.hidden {
      h2 {
        "Overview"
      }
      ol {
        li {
          h4 {
            "Operations"
          }
          p {
            "Has three params, the first is the operation and the other two are the values, exmaples:"
          }
          p {
            "Sum: "
          }
          (code(r#"robson
          comeu 0
          comeu 2
          comeu 4"#))
          p {
            "Subtraction:"
          }
          (code(r#"robson
          comeu 1
          comeu 5
          comeu 4"#))
          p {
            "Multiplication:"
          }
          (code(r#"robson
          comeu 2
          comeu 2
          comeu 3"#))
          p {
            "Division:"
          }
          (code(r#"robson
          comeu 3
          comeu 8
          comeu 2"#))
        }
        li {
          h4 {
            "If lower jump"
          }
          p {
            "Has three params, if the first param is lower than the seccond jump to third param"
          }
          (code(r#"robson robson
          comeu 1
          comeu 4
          lambeu :somealias"#))
        }
        li {
          h4 {
            "Push to the stack"
          }
          p {
            "Has one parameter, push its param to the stack, it has a
            handy syntax simplification on multiple push, example:"
          }
          (code(r#"robson robson robson
          comeu 10
          robson robson robson
          comeu 69"#))
          p {
            "Can be written as:"
          }
          (code(r#"robson robson robson
          comeu 10
          comeu 69"#))
        }
        li {
          h4 {
            "If equal"
          }
          p {
            "Has three  params, is the first param is equal to the third param, jump to third param, example:"
          }
          (code(r#"robson robson robson robson
          comeu 10
          comeu 10
          lambeu :somealias"#))
        }
        li {
          h4 {
            "Verify stack and jump"
          }
          p {
            "Has one parameter, if the stack is empty jump to the param, example:"
          }
          (code(r#"robson robson robson robson robson
          lambeu :somealias"#))
        }
        li {
          h4 {
            "Input user"
          }
          p {
            "Has three parameters, first the location to store the data,
            seccond its type (0 text, 1 usingned integer, 2 signed integer, 3 float ),
            third the limit that can be supplied for the user"
          }
          (code(r#"robson robson robson robson robson robson
          comeu 0
          comeu 0
          comeu 100"#))
        }
        li {
          h4 {
            "Print ASCII"
          }
          p {
            "Has no parameter, print the top of the stack as ascii, example:"
          }
          (code(r#"robson robson robson
          comeu 65
          robson robson robson robson robson robson robson"#))
        }
        li {
          h4 {
            "Print number"
          }
          p {
            "Has no parameter, print the top of the stack as a number, example:"
          }
          (code(r#"robson robson robson
          comeu 65
          robson robson robson robson robson robson robson robson"#))
        }
        li {
          h4 {
            "Set to memory"
          }
          p {
            "Has one parameter, set the top of the stack to the address of its parameter"
          }
          (code(r#"robson robson robson
          comeu 69
          robson robson robson robson robson robson robson robson robson robson
          comeu 0"#))
        }
        li {
          h4 {
            "Pop stack"
          }
          p {
            "Has no parameter, as it says, it pop the stack WOW"
          }
          (code("robson robson robson robson robson robson robson robson robson robson robson"))
        }
        li {
          h4 {
            "Load string"
          }
          p {
            "Has one parameter, read the memory values from address of parameter until '\\0', and adds it to stack in reverse"
          }
          (code(r#"robson robson robson robson robson robson robson robson robson robson robson robson
          comeu 0"#))
        }
        li {
          h4 {
            "Time"
          }
          p {
            "Wraps some time functions supported by robson"
          }
          p {
            "count, Will set the global time to start counting from the moment this command is run, if you run again it resets"
          }
          (code("robson robson robson robson robson robson robson robson robson robson robson robson robson\n comeu 0"))
          p {
            "interval, Set the global interval based on the stack top, for the time it will use the two values on stack top and put its binary in this order, [first stack top, seccond stack top]"
          }
          (code(r#"; Setting an inverval of 100ms
          robson robson robson
          comeu 100
          comeu 0
          robson robson robson robson robson robson robson robson robson robson robson robson robson
          comeu 1"#))
          p {
            "cmp_interval, Compare the interval with the global time, pushes to stack 0 if lower, 1 if equal and 2 if greater"
          }
          (code(r#"robson robson robson robson robson robson robson robson robson robson robson robson robson
          comeu 2"#))
        }
        li {
          h4 {
            "Flush"
          }
          p {
            "Will flush all pending to stdout"
          }
          (code("robson robson robson robson robson robson robson robson robson robson robson robson robson robson"))
        }
        li {
          h4 {
            "Terminal commands"
          }
          p {
            "The raw commands that robson supports"
          }
          p {
            "enter, Enable or disable the raw mode based on stack top, 0 to disable, 1 to enable"
          }
          (code(r#"robson robson robson
          comeu 1
          robson robson robson robson robson robson robson robson robson robson robson robson robson robson robson
          comeu 0"#))
          p {
            "clear, Will clear the terminal screen in one of two ways, based on stack top, if is 0, will purge the screen otherwise it will clear all"
          }
          (code(r#"robson robson robson
          comeu 1
          robson robson robson robson robson robson robson robson robson robson robson robson robson robson robson
          comeu 1"#))
          p {
            "poll, Will get the input of the user for the time specified on the stack top"
          }
          (code(r#"robson robson robson
          comeu 100
          comeu 0
          robson robson robson robson robson robson robson robson robson robson robson robson robson robson robson
          comeu 2"#))
          p {
            "cursor, Show or hide the cursor based on stack top"
          }
          (code(r#"robson robson robson
          comeu 0
          robson robson robson robson robson robson robson robson robson robson robson robson robson robson robson
          comeu 3"#))
          p {
            "mv_cursor, Will move the cursor to the coordinates on the two values of stack top, x, y respectively"
          }
          (code(r#"robson robson robson
          comeu 0
          comeu 1
          robson robson robson robson robson robson robson robson robson robson robson robson robson robson robson
          comeu 4"#))
          p {
            "color, Will set the color of the text that gets printed to the values of stack top, to reset the color, set the color to any value above 255."
            a.docs_link href="/static/ansi_color.png" {
              "Table with color values"
            }
          }
          (code(r#"robson robson robson
          comeu 120
          robson robson robson robson robson robson robson robson robson robson robson robson robson robson robson
          comeu 5"#))
        }
      }
    }

  }
}
