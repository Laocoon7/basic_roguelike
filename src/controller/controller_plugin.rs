use bevy::prelude::*;

use crate::controller::systems::handle_input;

pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input);
    }
}
