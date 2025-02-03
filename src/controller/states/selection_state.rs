use bevy::prelude::*;

/// Controls whether the user is selecting something
#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionState {
    /// Not selecting anything
    #[default]
    Disabled,
    /// Looking at something
    Look,
}
