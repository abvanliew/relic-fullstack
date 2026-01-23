use dioxus::prelude::*;
use manganis::{asset, Asset, AssetOptions, ImageFormat, ImageSize};
use manganis_core::AssetOptionsBuilder;

const IMG_SIZE_40_40: AssetOptionsBuilder<ImageAssetOptions> = AssetOptions::image()
  .with_size(ImageSize::Manual {
    width: 40,
    height: 40,
  })
  .with_format(ImageFormat::Avif);

pub const IMG_SELECTED: Asset = asset!("/assets/selected.png", IMG_SIZE_40_40);
pub const IMG_UNSELECTED: Asset = asset!("/assets/unselected.png", IMG_SIZE_40_40);
