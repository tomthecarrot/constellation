#[cfg(not(target_arch = "wasm32"))]
fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let mut app = circle_game::configure_app();
    app.run();

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("You can't run things this way on WASM")
}
