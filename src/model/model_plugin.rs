use bevy::prelude::*;

use crate::model::{
    components::{Player, Position},
    systems::spawn_player,
};

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>();
        app.register_type::<Position>();

        app.add_systems(Startup, spawn_player);
    }
}
