#![no_main]
#![no_std]

mod frames;
pub mod renderer;

use rmk::macros::rmk_central;

#[rmk_central]
mod keyboard_central {}
