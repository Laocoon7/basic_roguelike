use bevy::{
    ecs::{system::SystemState, world::Command},
    prelude::*,
};

use crate::{
    controller::{resources::CurrentSelection, selections::SelectionType, states::SelectionState},
    model::components::Position,
};

pub struct Look {
    pub position: Position,
}

impl Look {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}

impl Command for Look {
    fn apply(self, world: &mut World) {
        let mut state = SystemState::<(
            Commands,
            ResMut<CurrentSelection>,
            ResMut<NextState<SelectionState>>,
        )>::new(world);

        let (mut commands, mut current_selection, mut selection_state) = state.get_mut(world);

        current_selection.select(&mut commands, SelectionType::Single, self.position);
        selection_state.set(SelectionState::Look);

        state.apply(world);
    }
}
