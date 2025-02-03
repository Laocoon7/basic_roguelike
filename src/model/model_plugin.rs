use bevy::prelude::*;

use crate::model::{
    components::{Description, Player, Position},
    resources::CurrentMap,
    systems::spawn_player,
};

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Description>();
        app.register_type::<Player>();
        app.register_type::<Position>();

        app.register_type::<CurrentMap>();
        app.init_resource::<CurrentMap>();

        app.add_systems(Startup, spawn_player);
    }
}
