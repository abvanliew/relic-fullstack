use dioxus::prelude::*;

const DEFAULT_INCREMENT: f64 = 12.0;
const DEFAULT_EXTRA_HEIGHT: f64 = 18.0;

#[component]
pub fn StaggeredCell(
  children: Element, 
  #[props(default)] additional_classes: Option<String>,
  #[props(default)] increment_override: Option<f64>,
  #[props(default)] extra_height_override: Option<f64>,
) -> Element {
  let increment = increment_override.unwrap_or(DEFAULT_INCREMENT);
  let extra_height = extra_height_override.unwrap_or(DEFAULT_EXTRA_HEIGHT);
  let mut resized_height: Signal<Option<f64>> = use_signal(|| None);
  let height = resized_height().unwrap_or(0.0);
  let style = if height + extra_height < increment {
    format!( "_debug: height {height}, resized_height: {resized_height:?}, extra_height: {extra_height}, increment: {increment};" )
  } else {
    let spans = ((height + extra_height) / increment).ceil() as i32;
    format!(
      "grid-row: span {spans}; _debug: height {height}, resized_height: {resized_height:?}, extra_height: {extra_height}, increment: {increment};"
    )
  };
  let extra_class = match additional_classes {
    Some(class) => class,
    None => "".into(),
  };
  rsx!(
    div {
      class: "staggered-cell {extra_class}",
      style,
      onresize: move |event| {
        match event.data().get_content_box_size() {
          Ok(rect) => resized_height.set(Some(rect.height)),
          _ => ()
        }
      },
      {children}
    }
  )
}

#[component]
pub fn StaggeredGrid(
  children: Element,
  #[props(default, into)] class: String,
) -> Element {
  return rsx! {
    div { class: "staggered-grid {class}", {children} }
  }
}
