use bevy::prelude::*;

use crate::ui::{components::LogText, resources::GameLog};

pub fn update_log(log: Res<GameLog>, log_text: Single<&mut Text, With<LogText>>) {
    let mut log_text = log_text.into_inner();
    **log_text = log.iter().cloned().collect::<Vec<String>>().join("\n");
}
