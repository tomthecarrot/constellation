mod circle_bundle;

use circle_bundle::{BaselineKind, CircleBundle};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
mod wasm_main;

pub fn configure_app() -> bevy::app::AppBuilder {
    let mut app = App::build();
    app.insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system());

    app
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    const SQUARE_SIZE: usize = 10;

    for x in 0..SQUARE_SIZE {
        for y in 0..SQUARE_SIZE {
            fn offset_and_scale(val: usize) -> f32 {
                (val as f32 - SQUARE_SIZE as f32 / 2.0) * 50.
            }

            commands.spawn_bundle(CircleBundle::new(
                Transform::from_xyz(offset_and_scale(x), offset_and_scale(y), 1.),
                BaselineKind::Baseline,
            ));
            commands.spawn_bundle(CircleBundle::new(
                Transform::from_xyz(offset_and_scale(x), offset_and_scale(y), 0.),
                BaselineKind::BaselineFork,
            ));
        }
    }
}
