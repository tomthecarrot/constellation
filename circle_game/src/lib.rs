use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
mod wasm_main;
#[cfg(target_arch = "wasm32")]
pub use wasm_main::main;
