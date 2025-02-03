use bevy::prelude::*;

use crate::{
    controller::states::SelectionState,
    view::systems::{
        add_sprite_to_player, add_sprite_to_selectors, add_sprite_to_tile, position_to_transform,
    },
};

pub struct ViewPlugin;
impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                (add_sprite_to_player, add_sprite_to_tile),
                position_to_transform,
            ),
        );

        app.add_systems(
            PostUpdate,
            add_sprite_to_selectors.run_if(not(in_state(SelectionState::Disabled))),
        );
    }
}
