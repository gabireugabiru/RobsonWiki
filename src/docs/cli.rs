use maud::{html, PreEscaped};

use crate::components::code::code;

pub fn cli_section() -> PreEscaped<String> {
  html! {
    section._Cli {
      h2 { "Robson's cli"}
      p { "The robson cli gives you a lot of commands to helps you create and run your robson projects" }
      h2 {"Running .rbsn"}
      (code("$ robson [path_to_the_rbsn]"))
      h2 { "Compiling and running .robson"}
      (code("$ robson [path_to_the_robson]"))
      p {
        r#"For compiling a .robson you specify a .robson file and add the "compile" flag"#
      }
      (code("$ robson [path_to_robson] compile"))
      p { r#"For running is the same except we use the "run" flag"#}
      (code("$ robson [path_to_robson] run"))
      p { "All the .rbsn files are placed in the out folder inside your project." }
      p { "The general use for robson's cli is: "}
      (code("$ robson { [--flags] || [path_to_file] [?file_flags] }"))
      h2 {
        "--flags"
      }
      ol {
        li {
          "--version, display the robson installed version"
        }
        li {
          "--generate, input a string and convert it robson's push"
        }
        li {
          "--chars, press a key to get its ascii value"
        }
        li {
          "--boxes, a litlle showcase of box drawing characters"
        }
      }
      h2 {
        "file flags"
      }
      ol {
        li {
          "compile, will compile a .robson into a .rbsn"
        }
        li {
          "run, will compile then run a .robson"
        }
        li {
          "time, will show the run time of a .rbsn"
        }
        li {
          "print, will print the commands inside a .rbsn"
        }
      }
      button { "Next" }
    }
  }
}
