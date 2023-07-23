use bevy::prelude::*;

#[derive(Component)]
pub struct Name(&'static str);

impl Default for Name {
    fn default() -> Self {
        Self("Unnamed")
    }
}

impl Name {
    pub fn new(name: &'static str) -> Self {
        Self(name)
    }
}
