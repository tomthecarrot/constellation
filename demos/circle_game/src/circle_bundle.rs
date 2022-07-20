use crate::components::{tp, BaselineKind, ObjectHandle, PosHandle};

use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use lazy_static::lazy_static;

/// Allows the creation of the circles used in the game
#[derive(Bundle)]
pub struct CircleBundle {
    #[bundle]
    shape: ShapeBundle,
    baseline_kind: BaselineKind,
    object_handle: ObjectHandle,
    pos_handle: PosHandle,
}
impl CircleBundle {
    pub fn new(
        position: Transform,
        baseline_kind: BaselineKind,
        object_handle: ObjectHandle,
        pos_handle: PosHandle,
    ) -> Self {
        lazy_static! {
            static ref CIRCLE: shapes::Circle = shapes::Circle {
                radius: 5.0,
                ..Default::default()
            };
        }
        CircleBundle {
            shape: GeometryBuilder::build_as(
                &*CIRCLE,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(if baseline_kind.0 == tp::BaselineKind::Main {
                        Color::LIME_GREEN
                    } else {
                        Color::ORANGE_RED
                    }),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.5),
                },
                position,
            ),
            baseline_kind,
            object_handle,
            pos_handle,
        }
    }
}
