use wasm_bindgen::prelude::*;

use bevy::prelude::*;

#[wasm_bindgen(start)]
pub fn wasm_main() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let mut app = crate::configure_app();
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.run();

    Ok(())
}
