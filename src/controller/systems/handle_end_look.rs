use bevy::prelude::*;

use crate::{
    controller::{components::Selector, events::EndSelection, states::SelectionState},
    model::{
        components::{Description, Position},
        resources::CurrentMap,
    },
    ui::resources::GameLog,
};

pub fn handle_end_look(
    mut game_log: ResMut<GameLog>,
    current_map: Res<CurrentMap>,
    q_selection: Query<&Position, With<Selector>>,
    q_description: Query<&Description>,
    mut e_end_selection: EventReader<EndSelection>,
    mut selection_state: ResMut<NextState<SelectionState>>,
) {
    for end_selection in e_end_selection.read() {
        for &entity in end_selection.entities() {
            if let Ok(position) = q_selection.get(entity) {
                if let Some(entity) = current_map.get_terrain(*position) {
                    if let Ok(description) = q_description.get(entity) {
                        game_log.add_message(format!("Terrain: {}", &**description));
                    }
                }
                if let Some(entity) = current_map.get_actor(*position) {
                    if let Ok(description) = q_description.get(entity) {
                        game_log.add_message(format!("Actor: {}", &**description));
                    }
                }
            }
        }

        selection_state.set(SelectionState::Disabled);
    }
}
