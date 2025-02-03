use bevy::prelude::*;

use crate::controller::selections::Selection;

#[derive(Resource, Reflect, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub struct CurrentSelection(pub Selection);
