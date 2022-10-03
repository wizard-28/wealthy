#![allow(clippy::must_use_candidate)]

mod cell_widths;
mod color_triplet;
mod emoji_codes;
mod emoji_replace;
mod export_format;
mod filesize;
mod palette;
mod palettes;
mod ratio;
mod region;
mod terminal_theme;

pub mod cells;
pub mod measure;
// TODO: Re-export it as wealthy::progress::Spinners;
pub mod spinners;

pub use spinners::names as Spinners;
