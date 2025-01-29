use bevy::prelude::*;

use crate::ui::{
    components::{GameCamera, LogCamera, LogPanel, LogText, StatsCamera},
    resources::GameLog,
    systems::{setup_ui, spawn_camera, update_log},
};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameCamera>();
        app.register_type::<LogCamera>();
        app.register_type::<LogPanel>();
        app.register_type::<LogText>();
        app.register_type::<StatsCamera>();

        app.register_type::<GameLog>();
        app.init_resource::<GameLog>();

        app.add_systems(Startup, (spawn_camera, setup_ui).chain());
        app.add_systems(PostUpdate, update_log);
    }
}
