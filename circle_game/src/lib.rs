#![allow(clippy::diverging_sub_expression)]

mod circle;
mod circle_bundle;
mod components;

use crate::circle::Circle;
use crate::circle_bundle::CircleBundle;
use crate::components::{tp, BaselineKind, PosHandle};
use tp_client::contract::properties::dynamic::{DynTpPrimitive, DynTpProperty};
use tp_client::contract::Contract;
use tp_client::engine::Engine;
use tp_client::realm::{Realm, RealmID};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use eyre::WrapErr;

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
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(engine)
        .insert_resource(action_sender)
        .insert_resource(contract)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.chain(report_eyre))
        .add_system(random_walk.chain(report_eyre))
        .add_system(sync_transforms);
    app
}

fn report_eyre(In(err): In<eyre::Result<()>>) {
    err.unwrap()
}

fn setup(
    mut engine: ResMut<Engine>,
    contract: Res<Circle>,
    mut commands: Commands,
) -> eyre::Result<()> {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    const SQUARE_SIZE: usize = 10;

    for x in 0..SQUARE_SIZE {
        for y in 0..SQUARE_SIZE {
            fn offset_and_scale(val: usize) -> f32 {
                (val as f32 - SQUARE_SIZE as f32 / 2.0) * 50.
            }

            let x = offset_and_scale(x);
            let y = offset_and_scale(y);

            let baseline = engine.realm_mut().baseline_mut(tp::BaselineKind::Fork);

            let obj_handle = baseline
                .object_create(
                    &*contract,
                    [x, y]
                        .into_iter()
                        .map(DynTpPrimitive::from)
                        .map(DynTpProperty::from),
                    [].into_iter(),
                )
                .wrap_err("Failed to create object")?;

            let pos_handle = {
                let x_id = contract.states().x();
                let y_id = contract.states().y();

                let x_h = baseline
                    .bind_state(x_id, obj_handle)
                    .wrap_err("Failed to bind to object handle")?;
                let y_h = baseline
                    .bind_state(y_id, obj_handle)
                    .wrap_err("Failed to bind to object handle")?;

                PosHandle {
                    x: x_h.into(),
                    y: y_h.into(),
                }
            };

            // We need to deal with the main baseline eventually
            // commands.spawn_bundle(CircleBundle::new(
            //     Transform::from_xyz(x, y, 0.),
            //     tp::BaselineKind::Main.into(),
            //     obj_handle.into(),
            //     pos_handle,
            // ));

            commands.spawn_bundle(CircleBundle::new(
                Transform::from_xyz(x, y, 0.),
                tp::BaselineKind::Fork.into(),
                obj_handle.into(),
                pos_handle,
            ));
        }
    }
    Ok(())
}

fn random_walk(
    mut engine: ResMut<Engine>,
    mut query: Query<(&BaselineKind, &PosHandle)>,
) -> eyre::Result<()> {
    for (kind, pos) in query.iter_mut() {
        match kind.0 {
            tp::BaselineKind::Fork => (),
            _ => continue, // Shouldn't directly modify main baseline
        }

        let base = engine.realm_mut().baseline_mut(kind.0);

        base[pos.x].0 += (rand::random::<f32>() - 0.5) * 2.;
        base[pos.y].0 += (rand::random::<f32>() - 0.5) * 2.;
    }
    Ok(())
}

fn sync_transforms(
    engine: Res<Engine>,
    mut query: Query<(&BaselineKind, &PosHandle, &mut Transform)>,
) {
    for (kind, pos, mut transform) in query.iter_mut() {
        let base = engine.realm().baseline(kind.0);

        transform.translation.x = base[pos.x].0;
        transform.translation.y = base[pos.y].0;
    }
}
