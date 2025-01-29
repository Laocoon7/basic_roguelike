use bevy::prelude::*;

use crate::model::components::{Player, Position};

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, Position(IVec2::new(0, 0))));
}
