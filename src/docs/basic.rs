use maud::{html, PreEscaped};

use crate::components::memory::memory;

pub fn basic_section() -> PreEscaped<String> {
  html! {
    section._Basic.hidden {
      h2 {
        "Basic Concepts"
      }
      p {
        "Before continue to learn robson, first you need to understand concepts of
        memory and stack, if you're ok with that skip that section."
      }
      h2 { "Stack" }
      p {
        "The stack is basically an list of data that you access the top and adds to the top.<br>
        Exmaple:"
      }
      p { "We have a 65 on the stack." }
      div.stack {
        span { "65" }
      }
      p {
        "If we push 10 to the stack we will have:"
      }
      div.stack {
        span { "65" }
        span { "10" }
      }
      p { "We only can add to the top of the stack, not the bottom nor the middle." }
      p { "If we pop that stack we will remove the top of it getting back only the 65." }
      div.stack {
        "65"
      }
      p {
        "Like adding, you can just remove the top of the stack."
      }
      h2 {
        "Memory"
      }
      p {
        "The memory consists of data indexed by numbers like an vector/array"
        br;
        "Example:"
      }
      (memory(&[(0,0), (1,0), (2,0)]))

      p {
        "Unlike the stack, the memory you can read, add, and remove everything
        (within the scope of your application)"
      }
      p {
        "So we can set the address 2 with the number 69, the memory will be as following"
      }
      (memory(&[(0,0), (1,0), (2, 69)]))
      p {
        "If you set more than one time the same address you will replace its value"
      }
      p {
        "There's no way that you can remove entirely the value of the memory, so
        you set it to 0 in order to remove it"
      }
      button {
        "Next"
      }
    }
  }
}
