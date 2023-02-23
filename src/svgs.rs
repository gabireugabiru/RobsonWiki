use maud::{html, PreEscaped, Render};

pub enum Svgs {
  Logo,
  Curves,
  Hand,
}
impl Render for Svgs {
  fn render(&self) -> maud::Markup {
    let unsafe_html = match self {
      Self::Logo => include_str!("../static/Robson.svg"),
      Self::Curves => include_str!("../static/Curvazinha.svg"),
      Self::Hand => include_str!("../static/Mão.svg"),
    };

    let svg = PreEscaped(unsafe_html);

    html! {
      (svg)
    }
  }
}
