use bevy::prelude::*;

use crate::model::components::Position;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Position)]
pub struct Player;
