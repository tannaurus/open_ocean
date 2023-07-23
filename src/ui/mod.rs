use bevy::prelude::*;
pub mod ship;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum MenuState {
    Pause,
    Ship,
}

impl Default for MenuState {
    fn default() -> Self {
        Self::Ship
    }
}

impl MenuState {
    pub fn toggle_pause(current_state: &Self) -> Self {
        let updated_state = match current_state {
            Self::Pause => Self::Ship,
            _ => Self::Pause,
        };
        println!("Updated menu state 👉 {:?}", updated_state);
        updated_state
    }
}
