use maud::{html, PreEscaped, Render};

pub fn memory<T: Render>(indexes: &[(T, T)]) -> PreEscaped<String> {
  html! {
    div.memory {
        @for index in indexes {
            div {
                span.address {
                    (index.0)
                }
                span.value {
                    (index.1)
                }
            }
        }
    }
  }
}
