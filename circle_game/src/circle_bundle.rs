use tp_client::baseline::BaselineKind;
use tp_client::object::ObjectHandle;

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
}
impl CircleBundle {
    pub fn new(
        position: Transform,
        baseline_kind: BaselineKind,
        object_handle: ObjectHandle,
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
                ShapeColors::outlined(
                    if baseline_kind == BaselineKind::Main {
                        Color::LIME_GREEN
                    } else {
                        Color::ORANGE_RED
                    },
                    Color::BLACK,
                ),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default().with_line_width(0.5),
                },
                position,
            ),
            baseline_kind,
            object_handle,
        }
    }
}
