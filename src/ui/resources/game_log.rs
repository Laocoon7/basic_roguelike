use std::collections::VecDeque;

use bevy::prelude::*;

use crate::ui::UiConstants;

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct GameLog(pub VecDeque<String>);

impl Default for GameLog {
    fn default() -> Self {
        Self(VecDeque::with_capacity(UiConstants::LOG_LIMIT))
    }
}

impl GameLog {
    pub fn add_message(&mut self, message: impl ToString) {
        if self.0.len() >= UiConstants::LOG_LIMIT {
            self.0.pop_front();
        }
        self.0.push_back(message.to_string());
    }
}
