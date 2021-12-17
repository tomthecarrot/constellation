mod circle;
mod circle_bundle;

use circle_bundle::CircleBundle;
use tp_client::engine::Engine;
use tp_client::realm::{Realm, RealmID};
use tp_client::{baseline::BaselineKind, object::ObjectHandle};

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
    let (engine, action_sender) = Engine::new(Realm::new(RealmID::new("tmp id".to_string())), None);
    let mut app = App::build();
    app.insert_resource(Msaa { samples: 8 })
        .insert_resource(engine)
        .insert_resource(action_sender)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system());

    app
}

fn setup(engine: ResMut<Engine>, mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    const SQUARE_SIZE: usize = 10;

    for x in 0..SQUARE_SIZE {
        for y in 0..SQUARE_SIZE {
            fn offset_and_scale(val: usize) -> f32 {
                (val as f32 - SQUARE_SIZE as f32 / 2.0) * 50.
            }

            let x = offset_and_scale(x);
            let y = offset_and_scale(y);

            let obj_handle = todo!("Create a corresponding object in engine, and get handle");

            commands.spawn_bundle(CircleBundle::new(
                Transform::from_xyz(x, y, 1.),
                BaselineKind::Main,
                obj_handle,
            ));
            commands.spawn_bundle(CircleBundle::new(
                Transform::from_xyz(x, y, 0.),
                BaselineKind::Fork,
                obj_handle,
            ));
        }
    }
}

fn update_position(
    engine: Res<Engine>,
    mut query: Query<(&BaselineKind, &ObjectHandle, &mut Transform)>,
) {
    for (kind, obj_handle, transform) in query.iter_mut() {
        let base = match kind {
            BaselineKind::Main => engine.realm().baseline(BaselineKind::Main),

            BaselineKind::Fork => engine.realm().baseline(BaselineKind::Fork),
        };
        let obj = base.object(*obj_handle);
        let t = todo!("Read position from obj");
        *transform = t;
    }
}
