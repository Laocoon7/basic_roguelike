use bevy::prelude::*;

use crate::view::systems::{add_sprite_to_player, position_to_transform};

pub struct ViewPlugin;
impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (add_sprite_to_player, position_to_transform));
    }
}
