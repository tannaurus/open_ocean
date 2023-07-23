use bevy::prelude::*;

#[derive(PartialEq, PartialOrd)]
pub enum SailState {
    None,
    Mid,
    Full,
}

impl Default for SailState {
    fn default() -> Self {
        SailState::None
    }
}

impl SailState {
    pub fn speed_up(&self) -> Self {
        match &self {
            Self::None => Self::Mid,
            Self::Mid => Self::Full,
            Self::Full => Self::Full,
        }
    }

    pub fn slow_down(&self) -> Self {
        match &self {
            Self::None => Self::None,
            Self::Mid => Self::None,
            Self::Full => Self::Mid,
        }
    }

    pub fn as_forward_speed(&self, ship_forward: Vec3) -> Vec3 {
        match self {
            Self::None => Vec3::ZERO,
            Self::Mid => ship_forward * 2.0,
            Self::Full => ship_forward * 4.0,
        }
    }

    pub fn as_rotation_speed(&self) -> f32 {
        match self {
            Self::None => 0.1,
            Self::Mid => 0.5,
            Self::Full => 1.0,
        }
    }
}
