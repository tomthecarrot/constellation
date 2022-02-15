#![allow(clippy::diverging_sub_expression)]

mod circle;
mod circle_bundle;
mod components;

use circle::Circle;
use circle_bundle::CircleBundle;
use components::{tp, BaselineKind, ObjectHandle};
use tp_client::contract::properties::dynamic::{DynTpPrimitive, DynTpProperty};
use tp_client::contract::Contract;
use tp_client::engine::Engine;
use tp_client::realm::{Realm, RealmID};

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

pub fn configure_app() -> bevy::app::App {
    let (mut engine, action_sender) =
        Engine::new(Realm::new(RealmID::new("tmp id".to_string())), None);

    let contract: Circle = engine
        .realm_mut()
        .baseline_mut(tp::BaselineKind::Fork)
        .register_contract()
        .expect("contract failed to register");

    let mut app = App::new();
    app.insert_resource(Msaa { samples: 8 })
        .insert_resource(engine)
        .insert_resource(action_sender)
        .insert_resource(contract)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system(update_position);

    app
}

fn setup(mut engine: ResMut<Engine>, contract: Res<Circle>, mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    const SQUARE_SIZE: usize = 10;

    for x in 0..SQUARE_SIZE {
        for y in 0..SQUARE_SIZE {
            fn offset_and_scale(val: usize) -> f32 {
                (val as f32 - SQUARE_SIZE as f32 / 2.0) * 50.
            }

            let x = offset_and_scale(x);
            let y = offset_and_scale(y);

            let obj_handle = engine
                .realm_mut()
                .baseline_mut(tp::BaselineKind::Fork)
                .object_create(
                    &*contract,
                    [x, y]
                        .into_iter()
                        .map(DynTpPrimitive::from)
                        .map(DynTpProperty::from),
                    [].into_iter(),
                )
                .expect("Failed to create object");

            commands.spawn_bundle(CircleBundle::new(
                Transform::from_xyz(x, y, 1.),
                tp::BaselineKind::Main.into(),
                obj_handle.into(),
            ));
            commands.spawn_bundle(CircleBundle::new(
                Transform::from_xyz(x, y, 0.),
                tp::BaselineKind::Fork.into(),
                obj_handle.into(),
            ));
        }
    }
}

fn update_position(
    engine: Res<Engine>,
    contract: Res<Circle>,
    mut query: Query<(&BaselineKind, &ObjectHandle, &mut Transform)>,
) {
    for (kind, obj, mut transform) in query.iter_mut() {
        let obj = obj.0;
        let base = engine.realm().baseline(kind.0);
        let x_id = contract.states().x();
        let y_id = contract.states().y();

        let x_h = base.bind_state(x_id, obj).unwrap();
        let y_h = base.bind_state(y_id, obj).unwrap();

        let x = base[x_h].0;
        let y = base[y_h].0;

        transform.translation.x = x;
        transform.translation.y = y;
    }
}
