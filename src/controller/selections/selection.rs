use bevy::prelude::*;
use brt::grid_shapes::ShapeIter;

use crate::{
    controller::{components::Selector, selections::SelectionType},
    model::components::Position,
};

#[derive(Reflect, Clone, Default)]
pub struct Selection {
    pub entities: Vec<Entity>,
}

impl Selection {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn select(
        &mut self,
        commands: &mut Commands,
        selection_type: SelectionType,
        position: Position,
    ) {
        self.clear(commands);

        let offsets = match selection_type {
            SelectionType::Single => vec![(0, 0)],
            SelectionType::Circle(circle) => circle.iter().collect(),
            SelectionType::Line(line) => line.iter().collect(),
            SelectionType::RectangleBorder(rectangle) => rectangle.border_iter().collect(),
            SelectionType::RectangleFilled(rectangle) => rectangle.iter().collect(),
        };

        for offset in offsets {
            let position = position + offset;
            let entity = commands.spawn((Selector, position)).id();
            self.entities.push(entity);
        }
    }

    pub fn clear(&mut self, commands: &mut Commands) {
        for entity in self.entities.iter() {
            commands.entity(*entity).despawn_recursive();
        }

        self.entities.clear();
    }

    pub fn get_selection(&self) -> &Vec<Entity> {
        &self.entities
    }
}
