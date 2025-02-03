use bevy::prelude::*;

use crate::model::{
    components::{Description, Player, Position},
    resources::CurrentMap,
};

pub fn spawn_player(mut commands: Commands, mut current_map: ResMut<CurrentMap>) {
    let position = Position::new(1, 1);
    let entity = commands
        .spawn((Player, Description::new("Player"), position))
        .id();
    current_map.set_actor(position, Some(entity));
}
