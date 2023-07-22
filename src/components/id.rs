use bevy::prelude::*;

#[derive(Component)]
pub struct Name(&'static str);

impl Name {
    pub fn new(name: &'static str) -> Self {
        Self(name)
    }
}
