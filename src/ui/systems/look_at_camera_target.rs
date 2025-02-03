use bevy::prelude::*;

use crate::ui::components::{CameraTarget, GameCamera};

pub fn look_at_camera_target(
    maybe_camera_target: Option<Single<&Transform, (With<CameraTarget>, Without<GameCamera>)>>,
    game_camera: Single<&mut Transform, (With<GameCamera>, Without<CameraTarget>)>,
) {
    if let Some(camera_target) = maybe_camera_target {
        let camera_target = camera_target.into_inner();
        let mut game_camera = game_camera.into_inner();
        game_camera.translation.x = camera_target.translation.x;
        game_camera.translation.y = camera_target.translation.y;
    }
}
