use bevy::prelude::*;

use crate::{
    model::components::{Player, Position},
    ui::resources::GameLog,
};

pub fn handle_input(
    button_input: Res<ButtonInput<KeyCode>>,
    mut game_log: ResMut<GameLog>,
    mut q_player: Query<&mut Position, With<Player>>,
) {
    let mut x = 0;
    let mut y = 0;
    if button_input.just_pressed(KeyCode::Numpad1) {
        x -= 1;
        y -= 1;
        game_log.add_message("You walk southwest");
    }
    if button_input.just_pressed(KeyCode::Numpad2) {
        y -= 1;
        game_log.add_message("You walk south");
    }
    if button_input.just_pressed(KeyCode::Numpad3) {
        x += 1;
        y -= 1;
        game_log.add_message("You walk southeast");
    }
    if button_input.just_pressed(KeyCode::Numpad4) {
        x -= 1;
        game_log.add_message("You walk west");
    }
    if button_input.just_pressed(KeyCode::Numpad6) {
        x += 1;
        game_log.add_message("You walk east");
    }
    if button_input.just_pressed(KeyCode::Numpad7) {
        x -= 1;
        y += 1;
        game_log.add_message("You walk northwest");
    }
    if button_input.just_pressed(KeyCode::Numpad8) {
        y += 1;
        game_log.add_message("You walk north");
    }
    if button_input.just_pressed(KeyCode::Numpad9) {
        x += 1;
        y += 1;
        game_log.add_message("You walk northeast");
    }
    if x != 0 || y != 0 {
        for mut pos in q_player.iter_mut() {
            pos.0 += IVec2::new(x, y);
        }
    }
}
