use bevy::prelude::*;

use crate::{
    model::components::{Description, Position},
    ui::components::CameraTarget,
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Description, Position, CameraTarget)]
pub struct Player;
