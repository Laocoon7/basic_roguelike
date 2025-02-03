use bevy::prelude::*;

use crate::{
    controller::{events::CancelSelection, states::SelectionState},
    ui::resources::GameLog,
};

pub fn handle_cancel_selection(
    mut game_log: ResMut<GameLog>,
    mut e_cancel_selection: EventReader<CancelSelection>,
    mut selection_state: ResMut<NextState<SelectionState>>,
) {
    for _cancel_look in e_cancel_selection.read() {
        game_log.add_message("Nevermind");
        selection_state.set(SelectionState::Disabled);
    }
}
