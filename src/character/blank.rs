use dioxus::prelude::*;

use crate::common::HorizontalBar;
use crate::progression::prelude::BASE_DEFENSE;


#[component]
pub fn BlankSheet() -> Element {
  rsx! {
    div {
      class: "sheet-blank grid dim-attributes-wide",
      SheetHeading {}
      HorizontalBar {}
      div {
        class: "uv-capabilites",
        CapabilityBlock {}
        DefensesBlock {}
        div {
          class: "grid dim-rank-table",
          ResistancesBlock {}
        }
        BodyBlock {} 
      }
      div {
        class: "uv-defenses",
        div {
          class: "grid dim-rank-table",
          ExpertiseBlock {} 
        }
        div { class: "full subheading spacer-xlarge", "Resources" }
        FlowBlock { resource_count: 3 }
        FlowBlock { resource_count: 2 }
        FlowBlock { resource_count: 1 }
      }
    }
  }
}

#[component]
pub fn SheetHeading() -> Element {
  rsx! {
    div {
      class: "uv-other row",
      div {
        class: "row full align-center",
        div {
          class: "column",
          div { class: "highlight", "Paths:" }
          div { class: "highlight thick", "Trainings:" }
        }
        div {
          class: "row align-center content-right full",
          div { class: "highlight", "Level" }
          div { class: "digit-box-sum" }
        }
      }
    }
  }
}


#[component]
pub fn AttributeHeadings() -> Element {
  rsx! {
    div { class: "uv-total mini-text buffer", "Total" }
    div { class: "uv-rank mini-text buffer", "Ranks" }
    div { class: "uv-spec mini-text buffer", "Spec" }
    div { class: "uv-enchant mini-text buffer", "Enchant" }
  }
}


#[component]
pub fn CapabilityBlock() -> Element {
  rsx! {
    div {
      class: "grid dim-rank-table",
      div { class: "subheading uv-title justify-left buffer", "Capabilities" }
      AttributeHeadings {}
      CapabilityRow { title: "Physique", table_top: true }
      CapabilityRow { title: "Warfare" }
      CapabilityRow { title: "Spirit" }
      CapabilityRow { title: "Manipulation" }
    }
  }
}


#[component]
pub fn ExpertiseBlock() -> Element {
  rsx! {
    div { class: "subheading uv-title justify-left", "Expertise" }
    AttributeHeadings {}
    CapabilityRow { title: "", fillin_title: true, table_top: true }
    CapabilityRow { title: "", fillin_title: true }
    CapabilityRow { title: "", fillin_title: true }
    CapabilityRow { title: "", fillin_title: true }
    CapabilityRow { title: "", fillin_title: true }
    CapabilityRow { title: "", fillin_title: true }
    CapabilityRow { title: "", fillin_title: true }
    CapabilityRow { title: "", fillin_title: true }
    CapabilityRow { title: "", fillin_title: true }
  }
}


#[component]
pub fn CapabilityRow(
  title: &'static str,
  #[props(default)] fillin_title: bool,
  #[props(default)] table_top: bool,
) -> Element {
  let ( digit_sum_class, digit_class ) = match table_top {
    true => ("digit-box-sum", "digit-box border-ci"),
    false => ("digit-box-sum border-u", "digit-box border-l"),
  };
  rsx! {
    div { class: "uv-title justify-left pad-right", 
      div {
        class: if fillin_title { "fixed-title-width-large sheet-line underline-border" } else { "fixed-title-width sheet-line" }, 
        "{title}" 
      }
    }
    div { class: "uv-total",
      div { class: digit_sum_class }
    }
    div { class: "uv-rank",
      div { class: digit_class }
    }
    div { class: "uv-spec", 
      div { class: digit_class }
    }
    div { class: "uv-enchant", 
      div { class: digit_class }
    }
  }
}

#[component]
pub fn DefensesBlock() -> Element {
  rsx! {
    div {
      class: "grid dim-rank-table spacer-xlarge",
      div { class: "uv-title subheading justify-left", "Defenses" }
      AttributeHeadings {}
      DefenseRow { title: "Fortitude", table_top: true }
      DefenseRow { title: "Resolve" }
      DefenseRow { title: "Insight" }
      DefenseRow { title: "Tenacity" }
      DodgeBlock {}
    }
  }
}


#[component]
pub fn DefenseRow(
  title: &'static str,
  #[props(default)] table_top: bool,
) -> Element {
  let ( digit_sum_class, digit_base_class, digit_rank_class, digit_class ) = match table_top {
    true => (
      "digit-box-sum", 
      "digit-base sheet-line", 
      "digit-box", 
      "digit-box border-ci"
    ),
    false => (
      "digit-box-sum border-u", 
      "digit-base sheet-line border-und", 
      "digit-box border-u", 
      "digit-box border-l"
      ),
  };
  rsx! {
    div { class: "uv-title justify-left", "{title}" }
    div { class: "uv-total",
      div { class: digit_sum_class }
    }
    div { class: "uv-base", 
      div { class: digit_base_class, "{BASE_DEFENSE}" }
    }
    div { class: "uv-rank",
      div { class: digit_rank_class }
    }
    div { class: "uv-spec", 
      div { class: digit_class }
    }
    div { class: "uv-enchant", 
      div { class: digit_class }
    }
  }
}

#[component]
pub fn DodgeBlock() -> Element {
  rsx! {
    div {
      class: "uv-title subheading justify-left spacer-xlarge buffer",
      "Dodge & Armor"
    }
    div { class: "uv-total mini-text buffer anchor-down", "Total" }
    div {
      class: "uv-rank mini-text buffer centered anchor-down justify-center column",
      span { class: "arrow", "\u{2193}" } 
      br {}  "Tenacity" 
      br {} "Ranks"
    }
    div {
      class: "uv-spec mini-text buffer centered anchor-down justify-center column",
      span { class: "arrow", "\u{2193}" } 
      br {}  "Tenacity" 
      br {} "Spec"
    }
    div { class: "uv-enchant mini-text buffer anchor-down", "Enchant" }

    div { class: "uv-title justify-left", "Dodge" }
    div { class: "uv-total",
      div { class: "digit-box-sum" }
    }
    div { class: "uv-base",
      div { class: "digit-base sheet-line", "{BASE_DEFENSE}" }
    }
    div {
      class: "uv-rank",
      div { class: "digit-box calc-group-top calc-group-left" }
    }
    div {
      class: "uv-spec",
      div { class: "digit-box calc-group-top calc-group-right border-ci", }
    }
    div { class: "uv-enchant", 
      div { class: "digit-box border-ci" }
    }

    div { class: "uv-rank mini-text calc-group-left buffer", "Bulk" }
    div { class: "uv-spec mini-text calc-group-right buffer", "Drag" }

    div { class: "uv-title justify-left pad-right", 
      div { class: "sheet-line underline-border fixed-title-width" }
    }
    div {
      class: "uv-rank digit-box calc-group-left subtract-box",
      div { class: "minus-sign", "-" }
      div { class: "full" }
    }
    div {
      class: "uv-spec digit-box calc-group-right subtract-box border-ci",
      div { class: "minus-sign", "-" }
      div { class: "full" }
    }

    div { class: "uv-title mini-text justify-left buffer", "Armor" }
    div {
      class: "uv-calc-box mini-text calc-group-bottom calc-group-left calc-group-right buffer",
      "Minimum 0"
    }
  }
}


#[component]
pub fn ResistancesBlock() -> Element {
  return rsx!{
    div { class: "uv-full spacer" }
    div { class: "uv-total mini-text", "Total" }
    div { class: "uv-rank mini-text", "Armor" }
    div { class: "uv-spec mini-text", "Enchant" }
    ResistanceRow { title: "Physical Resistance", table_top: true }
    div { class: "uv-full spacer-xlarge" }
    div { class: "uv-title subheading justify-left", "Other Resistances" }
    ResistanceHeadings {}
    ResistanceRow { title: "", display: TitleDisplay::BlankEntry, table_top: true }
    ResistanceRow { title: "", display: TitleDisplay::BlankEntry }
    ResistanceRow { title: "", display: TitleDisplay::BlankEntry }
    ResistanceRow { title: "", display: TitleDisplay::BlankEntry }
  }
}


#[component]
pub fn ResistanceHeadings() -> Element {
  rsx! {
    div { class: "uv-total mini-text", "Total" }
    div { class: "uv-rank mini-text", "Base" }
    div { class: "uv-spec mini-text", "Enchant" }
  }
}


#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum TitleDisplay {
  #[default]
  Normal,
  BlankEntry,
}


#[component]
pub fn ResistanceRow(
  title: &'static str,
  #[props(default)] display: TitleDisplay,
  #[props(default)] table_top: bool,
) -> Element {
  let ( digit_sum_class, digit_class ) = match table_top {
    true => ("digit-box-sum", "digit-box border-ci"),
    false => ("digit-box-sum border-u", "digit-box border-l"),
  };
  rsx! {
    div { class: "uv-title justify-left pad-right", 
      div {
        class: match display {
          TitleDisplay::Normal => "fixed-title-width sheet-line",
          TitleDisplay::BlankEntry => "fixed-title-width underline-border try sheet-line",
        },
        "{title}" 
      }
    }
    div { class: "uv-total",
      div { class: digit_sum_class }
    }
    div { class: "uv-rank", 
      div { class: digit_class }
    }
    div { class: "uv-spec", 
      div { class: digit_class }
    }
  }
}


#[component]
pub fn BodyBlock() -> Element {
  return rsx!{
    div {
      class: "grid dim-rank-table",
      div { class: "uv-title subheading justify-left spacer-xlarge", "Body" }
      div { class: "uv-total mini-text buffer anchor-down", "Total" }
      div { class: "uv-rank mini-text buffer anchor-down", "Net Drag" }
      
      div {
        class: "uv-title justify-left pad-right",
        div { class: "fixed-title-width", "Speed" }
      }
      div { class: "uv-total",
        div { class: "digit-box-sum" }
      }
      div { class: "uv-base", 
        div { class: "digit-base sheet-line", "6" }
      }
      div {
        class: "uv-rank digit-box subtract-box",
        div { class: "minus-sign", "-" }
        div { class: "full" }
      }
      
      div {
        class: "uv-title justify-left pad-right",
        div { class: "fixed-title-width", "Dash" }
      }
      div { class: "uv-total",
        div { class: "digit-box-sum border-u" }
      }
      div { class: "uv-base",
        div { class: "digit-base sheet-line border-und", "3" }
      }
      div {
        class: "uv-rank digit-box border-u subtract-box",
        div { class: "minus-sign", "-" }
        div { class: "full" }
      }
    }

    div { class: "uv-full column align-start",
      div {
        class: "row align-center",
        div { class: "spacer",
          span { class: "highlight bumper", "Health" }
          ""
        }
      }
      div { class: "hp-box-big" }
    }
    div { class: "full justify-left spacer row-wrap spacer gap justify-left",
      div { class: "row",
        div { class: "highlight spacer-xsmall", "Constituion" }
        div { 
          class: "row-wrap gap",
          div { class: "box" }
          div { class: "box" }
          div { class: "box" }
          div { class: "box" }

          div { class: "box optional" }
          div { class: "box optional" }
          div { class: "box optional" }
          div { class: "box optional" }
        }
      }
    }
  }
}


#[component]
pub fn FlowBlock(resource_count: i32) -> Element {
  let span_count = resource_count + 1;
  return rsx!{
    div {
      class: "grid dim-flow-tracker-alt",
      div {
        class: "uv-title-flow highlight row",
        div { class: "full sheet-line underline-border underhang-2xsmall" }
        div { class: "digit-box" }
      }
      div {
        class: "uv-divider thin",
        style: "grid-row: span {span_count}"
      }
      div {
        class: "uv-reserves italics buffer anchor-down",
        "Reserves"
      }
      for _ in 0..resource_count { ResourceBlock {} }
      div { class: "uv-full short" }
    }
  }
}


#[component]
pub fn ResourceBlock() -> Element {
  return rsx!{
    div {
      class: "uv-title fixed-title-width sheet-line underline-border"
    }
    div {
      class: "uv-flow row-wrap min-height content-right gap-2xsmall under-buff",
      ResourcePotentials {}
    }
    div { 
      class: "uv-reserves row-wrap min-height gap-2xsmall under-buff",
      ResourcePotentials {}
      ResourcePotentials {}
      ResourcePotentials {}
    }
  }
}


#[component]
pub fn ResourcePotentials() -> Element {
  return rsx!{
    div { class: "box optional" }
    div { class: "box optional" }
    div { class: "box optional" }
    div { class: "box optional" }

    div { class: "box optional" }
    div { class: "box optional" }
    div { class: "box optional" }
    div { class: "box optional" }
  }
}
