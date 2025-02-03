use bevy::prelude::*;

#[derive(Event, Clone, Deref, DerefMut)]
pub struct EndSelection(pub Vec<Entity>);

impl EndSelection {
    pub fn new(entities: Vec<Entity>) -> Self {
        Self(entities)
    }

    pub fn entities(&self) -> &Vec<Entity> {
        &self.0
    }
}
