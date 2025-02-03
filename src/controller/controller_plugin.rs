use bevy::prelude::*;

use crate::controller::{
    components::Selector,
    events::{CancelSelection, EndSelection},
    resources::CurrentSelection,
    states::SelectionState,
    systems::{
        cleanup_selection, handle_cancel_selection, handle_end_look, handle_input,
        handle_look_input,
    },
};

pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Selector>();

        app.register_type::<CurrentSelection>();
        app.init_resource::<CurrentSelection>();

        app.init_state::<SelectionState>();

        app.add_event::<CancelSelection>();
        app.add_event::<EndSelection>();

        app.add_systems(
            Update,
            handle_input.run_if(in_state(SelectionState::Disabled)),
        );

        app.add_systems(
            Update,
            (handle_look_input, handle_end_look)
                .chain()
                .run_if(in_state(SelectionState::Look)),
        );

        app.add_systems(
            Update,
            handle_cancel_selection.run_if(not(in_state(SelectionState::Disabled))),
        );
        app.add_systems(OnEnter(SelectionState::Disabled), cleanup_selection);
    }
}
