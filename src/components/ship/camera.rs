use bevy::prelude::*;
use bevy_atmosphere::prelude::AtmosphereCamera;

pub const CAMERA_BASE_OFFSET: Vec3 = Vec3::new(0.0, 30.0, 60.0);

#[derive(Component)]
pub struct ShipCamera;

#[derive(Bundle)]
pub struct ShipCameraBundle {
    marker: ShipCamera,
    camera: Camera3dBundle,
    atmosphere: AtmosphereCamera,
    fog: FogSettings,
}

impl Default for ShipCameraBundle {
    fn default() -> Self {
        Self {
            marker: ShipCamera,
            camera: Camera3dBundle {
                transform: Transform::from_translation(CAMERA_BASE_OFFSET)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            atmosphere: AtmosphereCamera::default(),
            fog: FogSettings {
                color: Color::rgba(0.6, 0.6, 0.6, 1.0),
                falloff: FogFalloff::Exponential { density: 0.0003 },
                ..default()
            },
        }
    }
}
