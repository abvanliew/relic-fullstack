use dioxus::prelude::*;

const IMG_SIZE_40_40: ImageAssetOptions = ImageAssetOptions::new()
  .with_size(ImageSize::Manual {
    width: 40,
    height: 40,
  })
  .with_format(ImageFormat::Avif);

pub const IMG_SELECTED: Asset = asset!("/assets/selected.png", IMG_SIZE_40_40);
pub const IMG_UNSELECTED: Asset = asset!("/assets/unselected.png", IMG_SIZE_40_40);
