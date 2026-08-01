use dioxus::prelude::*;

use crate::character::sheet::CharacterSheet;
use crate::common::HorizontalBar;
use crate::progression::prelude::BASE_DEFENSE;

#[component]
pub fn FillableSheet(#[props(default)] character_sheet: Option<CharacterSheet>) -> Element {
  let character_name = character_sheet.as_ref().map(|c| c.name.clone());

  // capabilities
  let physique = character_sheet
    .as_ref()
    .and_then(|c| c.attributes.physique.clone());
  let warfare = character_sheet
    .as_ref()
    .and_then(|c| c.attributes.warfare.clone());
  let spirit = character_sheet
    .as_ref()
    .and_then(|c| c.attributes.spirit.clone());
  let manipulation = character_sheet
    .as_ref()
    .and_then(|c| c.attributes.manipulation.clone());

  // defenses
  let fortitude = character_sheet
    .as_ref()
    .and_then(|c| c.attributes.fortitude.clone());
  let insight = character_sheet
    .as_ref()
    .and_then(|c| c.attributes.insight.clone());
  let resolve = character_sheet
    .as_ref()
    .and_then(|c| c.attributes.resolve.clone());
  let dodge = character_sheet
    .as_ref()
    .and_then(|c| c.attributes.dodge.clone());

  rsx! {
    div {
      class: "sheet-blank grid dim-attributes-wide",
      SheetHeading { level: None, character_name }
      HorizontalBar {}
      div {
        class: "uv-capabilites",
        CapabilityBlock { physique, warfare, spirit, manipulation }
        DefensesBlock { fortitude, resolve, insight, dodge }
        ResistancesBlock {}
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
      }
    }
  }
}

#[component]
pub fn SheetHeading(
  #[props(default)] level: Option<i32>, #[props(default)] character_name: Option<String>,
) -> Element {
  let level = level.map_or("".into(), |l| l.to_string());
  let character_name = character_name.unwrap_or("".into());
  rsx! {
    div {
      class: "uv-capabilities row align-center row-height",
      div { class: "highlight", "Character:" }
      div { class: "heavier", "{character_name}" }
    }
    div {
      class: "uv-defenses row align-center row-height",
      div { class: "highlight", "Paths:" }
      div { class: "heavier", "" }
      div { class: "highlight align-right", "Level" }
      div { class: "digit-box-sum", "{level}" }
    }
    div {
      class: "uv-capabilities row align-center row-height",
      div { class: "highlight", "Description:" }
      div { class: "heavier", "" }
    }
    div {
      class: "uv-defenses row align-center row-height",
      div { class: "highlight", "Trainings:" }
      div { class: "heavier", "" }
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
pub fn CapabilityBlock(
  #[props(default)] physique: Option<i32>, #[props(default)] warfare: Option<i32>,
  #[props(default)] spirit: Option<i32>, #[props(default)] manipulation: Option<i32>,
) -> Element {
  rsx! {
    div {
      class: "grid dim-rank-table",
      div { class: "subheading uv-title justify-left buffer", "Capabilities" }
      AttributeHeadings {}
      CapabilityRow { title: "Physique", value: physique, table_top: true, }
      CapabilityRow { title: "Warfare", value: warfare, }
      CapabilityRow { title: "Presence", value: spirit, }
      CapabilityRow { title: "Manipulation", value: manipulation, }
    }
  }
}

#[component]
pub fn ExpertiseBlock() -> Element {
  rsx! {
    div { class: "subheading uv-title justify-left fixed-title-width-large", "Expertise" }
    AttributeHeadings {}
    CapabilityRow { fillin_title: true, table_top: true }
    CapabilityRow { fillin_title: true }
    CapabilityRow { fillin_title: true }
    CapabilityRow { fillin_title: true }
    CapabilityRow { fillin_title: true }
    CapabilityRow { fillin_title: true }
    CapabilityRow { fillin_title: true }
    CapabilityRow { fillin_title: true }
  }
}

#[component]
pub fn CapabilityRow(
  #[props(into)]
  #[props(default)]
  title: String, #[props(default)] value: Option<i32>,
  #[props(default)] fillin_title: bool, #[props(default)] table_top: bool,
) -> Element {
  let (digit_sum_class, digit_class) = match table_top {
    true => ("digit-box-sum", "digit-box border-ci"),
    false => ("digit-box-sum border-u", "digit-box border-l"),
  };
  rsx! {
    div { class: "uv-title justify-left",
      div {
        class: if fillin_title { "sheet-line align-center bumper underline-border" } else { "sheet-line align-center bumper" },
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
pub fn DefensesBlock(
  #[props(default)] fortitude: Option<i32>, #[props(default)] resolve: Option<i32>,
  #[props(default)] insight: Option<i32>, #[props(default)] dodge: Option<i32>,
) -> Element {
  rsx! {
    div {
      class: "grid dim-rank-table spacer-xlarge",
      div { class: "uv-title subheading justify-left", "Defenses & Armor" }
      AttributeHeadings {}
      DefenseRow { title: "Fortitude", value: fortitude, table_top: true }
      DefenseRow { title: "Resolve", value: resolve, }
      DefenseRow { title: "Insight", value: insight, }
      DodgeBlock {}
    }
  }
}

#[component]
pub fn DefenseRow(
  title: &'static str, #[props(default)] value: Option<i32>, #[props(default)] table_top: bool,
) -> Element {
  let value = value.map_or("".into(), |l| l.to_string());
  let (digit_sum_class, digit_base_class, digit_rank_class, digit_class) = match table_top {
    true => (
      "digit-box-sum",
      "digit-base sheet-line",
      "digit-box",
      "digit-box border-ci",
    ),
    false => (
      "digit-box-sum border-u",
      "digit-base sheet-line border-und",
      "digit-box border-u",
      "digit-box border-l",
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
      div { class: digit_rank_class, "{value}" }
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

    div { class: "uv-title justify-left", "Dodge" }
    div { class: "uv-total",
      div { class: "digit-box-sum border-u" }
    }
    div { class: "uv-base",
      div { class: "digit-base sheet-line border-und", "{BASE_DEFENSE}" }
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
      div { class: "digit-box border-l" }
    }

    div {
      class: "uv-calc-box mini-text calc-group-left calc-group-right buffer", "Bulk"
    }

    div { class: "uv-title-base sheet-line underline-border" }
    div {
      class: "uv-calc-box calc-group-left calc-group-right",
      div {
        class: "digit-box subtract-box",
        div { class: "minus-sign", "-" }
        div { class: "full" }
      }
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
  return rsx! {
    div {
      class: "grid dim-rank-table",
      div { class: "uv-full spacer" }
      div { class: "uv-total mini-text", "Total" }
      div { class: "uv-rank mini-text", "Armor" }
      div { class: "uv-spec mini-text", "Enchant" }
      ResistanceRow { title: "Physical Resistance", table_top: true }
      div { class: "uv-full spacer-xlarge" }
      div { class: "uv-title subheading justify-left", "Other Resistances" }
      ResistanceHeadings {}
      ResistanceRow { display: TitleDisplay::BlankEntry, table_top: true }
      ResistanceRow { display: TitleDisplay::BlankEntry }
      ResistanceRow { display: TitleDisplay::BlankEntry }
    }
  };
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
  #[props(into)]
  #[props(default)]
  title: String, #[props(default)] display: TitleDisplay,
  #[props(default)] table_top: bool,
) -> Element {
  let (digit_sum_class, digit_class) = match table_top {
    true => ("digit-box-sum", "digit-box border-ci"),
    false => ("digit-box-sum border-u", "digit-box border-l"),
  };
  rsx! {
    div {
      class: "uv-title justify-left",
      div {
        class: match display {
          TitleDisplay::Normal => "sheet-line align-center bumper",
          TitleDisplay::BlankEntry => "sheet-line align-center bumper underline-border",
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

#[derive(Debug, Default, Clone, PartialEq)]
pub enum SpeedCalc {
  #[default]
  SixWithDrag,
  #[allow(unused)]
  FlatThree,
}

#[component]
pub fn SpeedBlock(#[props(default)] speed_calc: SpeedCalc) -> Element {
  return rsx! {
    div { class: "uv-total mini-text buffer anchor-down", "Total" }
    match &speed_calc {
      SpeedCalc::SixWithDrag => rsx! {
        div { class: "uv-rank mini-text buffer anchor-down", "Drag" }
        div {
          class: "uv-title justify-left pad-right",
          div { class: "fixed-title-width", "Speed" }
        }
        div { class: "uv-total",
          div { class: "digit-box-sum highlight", "" }
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
          class: "uv-rank digit-box subtract-box border-u",
          div { class: "minus-sign", "-" }
          div { class: "full" }
        }
      },
      SpeedCalc::FlatThree => rsx! {
        div {
          class: "uv-title justify-left pad-right",
          div { class: "fixed-title-width", "Speed" }
        }
        div { class: "uv-total",
          div { class: "digit-box-sum highlight", "3" }
        }

        div { class: "uv-rank mini-text buffer anchor-down", "Drag" }
        div {
          class: "uv-title justify-left pad-right",
          div { class: "fixed-title-width", "Dash" }
        }
        div { class: "uv-total",
          div { class: "digit-box-sum border-u" }
        }
        div { class: "uv-base",
          div { class: "digit-base sheet-line", "3" }
        }
        div {
          class: "uv-rank digit-box subtract-box",
          div { class: "minus-sign", "-" }
          div { class: "full" }
        }
      },
    }
  };
}

#[component]
pub fn BodyBlock() -> Element {
  return rsx! {
    div {
      class: "grid dim-rank-table",
      div { class: "uv-title subheading justify-left spacer-xlarge", "Body" }

      SpeedBlock { speed_calc: SpeedCalc::SixWithDrag }

      div { class: "uv-title justify-left spacer", "Health" }
      div { class: "uv-total spacer digit-box-sum" }
    }

    div { class: "uv-full column align-start spacer",
      div { class: "hp-box-big" }
    }
    div { class: "full justify-left spacer row-wrap spacer gap justify-left",
      div { class: "row",
        div { class: "highlight spacer-xsmall", "Constituion" }
        div {
          class: "row-wrap gap-xsmall",
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
  };
}

#[component]
pub fn FlowBlock(resource_count: i32) -> Element {
  let span_count = resource_count + 1;
  return rsx! {
    div {
      class: "grid dim-flow-tracker-alt",
      div {
        class: "uv-full highlight row",
        div { class: "full sheet-line underline-border underhang-2xsmall" }
        div { class: "digit-box" }
      }
      div {
        class: "uv-flow italics buffer anchor-down",
        "Reserves"
      }
      div {
        class: "uv-divider thin",
        style: "grid-row: span {span_count}"
      }
      for _ in 0..resource_count { ResourceBlock {} }
      div { class: "uv-full short" }
    }
  };
}

#[component]
pub fn ResourceBlock() -> Element {
  return rsx! {
    div {
      class: "uv-title fixed-title-width sheet-line underline-border"
    }
    div {
      class: "uv-flow row-wrap min-height content-right gap-2xsmall under-buff",
      ResourcePotentials {}
      ResourcePotentials {}
      ResourcePotentials {}
    }
    div {
      class: "uv-reserves row-wrap min-height gap-2xsmall under-buff",
      ResourcePotentials {}
    }
  };
}

#[component]
pub fn ResourcePotentials() -> Element {
  return rsx! {
    div { class: "box optional" }
    div { class: "box optional" }
    div { class: "box optional" }
    div { class: "box optional" }

    div { class: "box optional" }
    div { class: "box optional" }
    div { class: "box optional" }
    div { class: "box optional" }
  };
}
