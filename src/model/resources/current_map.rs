use bevy::prelude::*;

use crate::model::{maps::Map, ModelConstants};

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct CurrentMap(pub Map);

impl FromWorld for CurrentMap {
    fn from_world(world: &mut World) -> Self {
        let mut commands = world.commands();
        Self(Map::new(
            &mut commands,
            (ModelConstants::MAP_WIDTH, ModelConstants::MAP_HEIGHT),
        ))
    }
}
