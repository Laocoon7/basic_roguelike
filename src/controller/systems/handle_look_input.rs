use bevy::prelude::*;
use brt::direction::Direction;

use crate::{
    controller::{
        components::Selector,
        events::{CancelSelection, EndSelection},
        resources::CurrentSelection,
    },
    model::components::Position,
};

pub fn handle_look_input(
    button_input: Res<ButtonInput<KeyCode>>,
    current_selection: Res<CurrentSelection>,
    mut e_cancel_selection: EventWriter<CancelSelection>,
    mut e_end_selection: EventWriter<EndSelection>,
    q_selection: Single<&mut Position, With<Selector>>,
) {
    if button_input.just_pressed(KeyCode::Escape) {
        e_cancel_selection.send(CancelSelection);
        return;
    }

    if button_input.just_pressed(KeyCode::Enter) || button_input.just_pressed(KeyCode::NumpadEnter)
    {
        e_end_selection.send(EndSelection::new(current_selection.get_selection().clone()));
        return;
    }

    let mut look_position = q_selection.into_inner();

    let mut dir = Direction::NONE;

    if button_input.just_pressed(KeyCode::Numpad1) {
        dir += Direction::SOUTH_WEST;
    }
    if button_input.just_pressed(KeyCode::Numpad2) {
        dir += Direction::SOUTH;
    }
    if button_input.just_pressed(KeyCode::Numpad3) {
        dir += Direction::SOUTH_EAST;
    }
    if button_input.just_pressed(KeyCode::Numpad4) {
        dir += Direction::WEST;
    }
    if button_input.just_pressed(KeyCode::Numpad6) {
        dir += Direction::EAST;
    }
    if button_input.just_pressed(KeyCode::Numpad7) {
        dir += Direction::NORTH_WEST;
    }
    if button_input.just_pressed(KeyCode::Numpad8) {
        dir += Direction::NORTH;
    }
    if button_input.just_pressed(KeyCode::Numpad9) {
        dir += Direction::NORTH_EAST;
    }
    dir.simplify();
    if dir != Direction::NONE {
        *look_position += dir;
        info!("look_position: {:?}", *look_position);
    }
}
