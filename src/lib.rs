#![warn(rust_2018_idioms)]
#![allow(dead_code)]
#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]

pub mod core;
pub mod graphics;
pub mod gui;

mod application;
mod assets;
mod winit_state;

pub use application::{ Application, AppState };
pub use assets::AssetManager;
pub use winit_state::WinitState;