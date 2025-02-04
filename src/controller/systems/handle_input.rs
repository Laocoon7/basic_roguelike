use bevy::prelude::*;
use brt::direction::Direction;

use crate::{
    controller::commands::Look,
    model::{
        commands::TryMove,
        components::{Player, Position},
    },
    ui::resources::GameLog,
};

pub fn handle_input(
    mut commands: Commands,
    button_input: Res<ButtonInput<KeyCode>>,
    mut game_log: ResMut<GameLog>,
    player: Single<(Entity, &Position), With<Player>>,
) {
    let (player_entity, player_position) = player.into_inner();

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
        game_log.add_message(format!("You walk {}", dir));
        commands.entity(player_entity).queue(TryMove::new(dir));
    }

    if button_input.just_pressed(KeyCode::KeyL) {
        game_log.add_message("You look around");
        commands.queue(Look::new(*player_position));
    }
}
