use bevy::{color::palettes::css, prelude::*, render::camera::Viewport, window::PrimaryWindow};

use crate::{
    model::ModelConstants,
    ui::{
        components::{GameCamera, LogCamera, StatsCamera},
        UiConstants,
    },
    view::ViewConstants,
};

pub fn spawn_camera(mut commands: Commands, primary_window: Single<&Window, With<PrimaryWindow>>) {
    let window = primary_window.into_inner();

    let stats_viewport = Viewport {
        physical_position: UVec2::new(0, 0),
        physical_size: UVec2::new(UiConstants::STATS_WIDTH, window.height() as u32),
        ..Default::default()
    };

    let log_viewport = Viewport {
        physical_position: UVec2::new(0, window.height() as u32 - UiConstants::LOG_HEIGHT),
        physical_size: UVec2::new(window.width() as u32, UiConstants::LOG_HEIGHT),
        ..Default::default()
    };

    let game_viewport = Viewport {
        physical_position: UVec2::new(UiConstants::STATS_WIDTH, 0),
        physical_size: UVec2::new(
            window.width() as u32 - UiConstants::STATS_WIDTH,
            window.height() as u32 - UiConstants::LOG_HEIGHT,
        ),
        ..Default::default()
    };

    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(game_viewport),
            clear_color: ClearColorConfig::Custom(Color::Srgba(css::BLACK)),
            ..Default::default()
        },
        Msaa::Off,
        Transform::from_xyz(
            ModelConstants::MAP_WIDTH as f32 / 2.0 * ViewConstants::TILE_SIZE,
            ModelConstants::MAP_HEIGHT as f32 / 2.0 * ViewConstants::TILE_SIZE,
            0.0,
        ),
        UiConstants::GAME_LAYER,
        GameCamera,
    ));

    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            viewport: Some(stats_viewport),
            clear_color: ClearColorConfig::Custom(Color::Srgba(UiConstants::STATS_BACKGROUND)),
            ..Default::default()
        },
        Msaa::Off,
        UiConstants::UI_LAYER,
        StatsCamera,
    ));

    commands.spawn((
        Camera2d,
        Camera {
            order: 2,
            viewport: Some(log_viewport),
            clear_color: ClearColorConfig::Custom(Color::Srgba(UiConstants::LOG_BACKGROUND)),
            ..Default::default()
        },
        Msaa::Off,
        UiConstants::UI_LAYER,
        LogCamera,
    ));
}
