
// #[cfg(target_arch = "wasm32")]
// mod wasm_main;
// #[cfg(target_arch = "wasm32")]
// pub use wasm_main::main;


#[cfg(not(target_arch = "wasm32"))]
mod regular_main;
#[cfg(not(target_arch = "wasm32"))]
pub use regular_main::main;
