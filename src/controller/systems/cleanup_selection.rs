use bevy::prelude::*;

use crate::controller::resources::CurrentSelection;

pub fn cleanup_selection(mut commands: Commands, mut selection: ResMut<CurrentSelection>) {
    selection.clear(&mut commands);
}
