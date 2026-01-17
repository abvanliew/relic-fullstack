use std::rc::Rc;

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
  let increment = increment_override.unwrap_or( DEFAULT_INCREMENT );
  let extra_height = extra_height_override.unwrap_or( DEFAULT_EXTRA_HEIGHT );

  let mut element_data: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
  let rect_future = use_resource( move || async move {
    match element_data() {
      Some( x) => { Some( x.get_client_rect().await ) },
      None => None,
    }
  } );
  let height = match &*rect_future.read_unchecked() {
    Some( Some( Ok( x ) ) ) => x.height(),
    _ => 0.0,
  };
  let span_style = if height < increment {
    format!( "margin-bottom: {}px;", extra_height )
  } else {
    let spans = ( ( height + extra_height ) / increment ).ceil() as i32;
    format!( "margin-bottom: {}px; grid-row: span {};", extra_height, spans )
  };
  let extra_class = match additional_classes {
    Some( class ) => class,
    None => "".into(),
  };
  rsx!(
    div {
      class: "staggered-cell {extra_class}",
      style: span_style,
      onmounted: move |element| element_data.set( Some( element.data() ) ),
      {children}
    }
  )
}
