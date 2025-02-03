use bevy::prelude::*;

use crate::model::components::Description;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Description)]
pub enum TerrainType {
    #[default]
    Floor,
    Wall,
}
