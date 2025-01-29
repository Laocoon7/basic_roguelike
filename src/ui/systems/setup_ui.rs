use bevy::prelude::*;

use crate::ui::{
    components::{LogCamera, LogPanel, LogText, StatsCamera},
    UiConstants,
};

pub fn setup_ui(
    mut commands: Commands,
    log_camera: Single<Entity, With<LogCamera>>,
    stats_camera: Single<Entity, With<StatsCamera>>,
) {
    setup_log(&mut commands, log_camera.into_inner());
    setup_stats(&mut commands, stats_camera.into_inner());
}

fn setup_log(commands: &mut Commands, log_camera: Entity) {
    commands
        .spawn((
            TargetCamera(log_camera),
            Node {
                height: Val::Px(UiConstants::LOG_HEIGHT as f32),
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::ColumnReverse, // Newest at bottom
                padding: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
            BackgroundColor(Color::Srgba(UiConstants::LOG_BACKGROUND)),
            LogPanel,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Log:"),
                TextColor(Color::Srgba(UiConstants::LOG_FOREGROUND)),
                LogText,
            ));
        });
}

fn setup_stats(commands: &mut Commands, stats_camera: Entity) {
    commands
        .spawn((
            TargetCamera(stats_camera),
            Node {
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("Stats:"));
        });
}
