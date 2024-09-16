use cfg_if::cfg_if;
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;


mod state;
mod renderer;
mod application;
pub mod engine;
pub mod window;