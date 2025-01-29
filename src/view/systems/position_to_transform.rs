use bevy::prelude::*;

use crate::{model::components::Position, view::ViewConstants};

pub fn position_to_transform(mut q_objects: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in q_objects.iter_mut() {
        transform.translation = Vec3::new(
            position.x() as f32 * ViewConstants::TILE_SIZE,
            position.y() as f32 * ViewConstants::TILE_SIZE,
            0.0,
        );
    }
}
