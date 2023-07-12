use bevy::prelude::*;

#[derive(Component)]
pub struct Name(String);

impl Name {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}
