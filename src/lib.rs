#![allow(clippy::must_use_candidate)]

mod cell_widths;
mod emoji_codes;
mod emoji_replace;
mod export_format;
mod filesize;
mod palettes;
mod ratio;
mod region;

pub mod cells;
pub mod color_triplet;
pub mod measure;
pub mod palette;
// TODO: Re-export it as wealthy::progress::Spinners;
pub mod spinners;

pub use spinners::names as Spinners;
