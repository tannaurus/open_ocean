use std::f32::consts::TAU;

use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_atmosphere::prelude::*;

use crate::id::Name;

const SHIP_SPEED: f32 = 6.0;
// Base ship turn speed. Will be modified by the ship's velocity.
const SHIP_TURN_SPEED: (f32, f32) = (-0.05, 0.05);
const CAMERA_SPEED: f32 = 1.0;
const CAMERA_MAX_HEIGHT: f32 = 40.0;
// Max distance camera can pan left and right
const CAMERA_MAX_PAN: f32 = 40.0;

const CAMERA_BASE_OFFSET: Vec3 = Vec3::new(0.0, 30.0, 60.0);

#[derive(Bundle)]
pub struct ShipBundle {
    name: Name,
    ship: Ship,
}

#[derive(PartialEq, PartialOrd)]
enum SailState {
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
    fn speed_up(&self) -> Self {
        match &self {
            Self::None => Self::Mid,
            Self::Mid => Self::Full,
            Self::Full => Self::Full,
        }
    }

    fn slow_down(&self) -> Self {
        match &self {
            Self::None => Self::None,
            Self::Mid => Self::None,
            Self::Full => Self::Mid,
        }
    }

    fn as_forward_speed(&self, ship_forward: Vec3) -> Vec3 {
        match self {
            Self::None => Vec3::ZERO,
            Self::Mid => ship_forward * 2.0,
            Self::Full => ship_forward * 4.0,
        }
    }

    fn as_rotation_speed(&self) -> f32 {
        match self {
            Self::None => 0.1,
            Self::Mid => 0.5,
            Self::Full => 1.0,
        }
    }
}

#[derive(Component, Default)]
pub struct Ship {
    sails: SailState,
}

#[derive(Component)]
pub struct ShipCamera;

/// A child component of the ship that remains in a fixed relative position
/// above the model. [ShipCamera] uses this component's location to determine its X bounds.
#[derive(Component)]
pub struct ShipFollower;

pub struct Systems;
impl Systems {
    pub fn spawn_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
        let ship_handle =
            asset_server.load("models/pirate_ship/dutch_ship_large_01_1k.gltf#Scene0");

        // spawn ship components + model
        commands
            .spawn((
                ShipBundle {
                    name: Name::new("Eleanor"),
                    ship: Default::default(),
                },
                SpatialBundle {
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((SceneBundle {
                    scene: ship_handle.clone(),
                    // Rotate ship model to line up with rotation axis.
                    transform: Transform::from_rotation(Quat::from_rotation_y(
                        std::f32::consts::FRAC_PI_2,
                    )),
                    ..Default::default()
                },));
                // spawn ship camera
                parent.spawn((
                    ShipCamera,
                    Camera3dBundle {
                        transform: Transform::from_translation(CAMERA_BASE_OFFSET)
                            .looking_at(Vec3::ZERO, Vec3::Y),
                        ..default()
                    },
                    AtmosphereCamera::default(),
                    FogSettings {
                        color: Color::rgba(0.6, 0.6, 0.6, 1.0),
                        falloff: FogFalloff::Exponential { density: 0.0003 },
                        ..default()
                    },
                ));
            });
    }

    pub fn ship_controller(
        mut ship: Query<(&mut Transform, &mut Ship), (With<Ship>, Without<ShipCamera>)>,
        mut camera: Query<&mut Transform, (With<ShipCamera>, Without<Ship>)>,
        mut mouse: EventReader<MouseMotion>,
        keyboard: Res<Input<KeyCode>>,
        time: Res<Time>,
    ) {
        let (mut ship, mut ship_state) = ship.single_mut();
        let mut camera = camera.single_mut();

        // Change sails position
        if keyboard.just_pressed(KeyCode::W) {
            ship_state.sails = ship_state.sails.speed_up();
        } else if keyboard.just_pressed(KeyCode::S) {
            ship_state.sails = ship_state.sails.slow_down();
        }

        if keyboard.pressed(KeyCode::D) {
            ship.rotate_y(
                SHIP_TURN_SPEED.0
                    * ship_state.sails.as_rotation_speed()
                    * TAU
                    * time.delta_seconds(),
            );
        }
        if keyboard.pressed(KeyCode::A) {
            ship.rotate_y(
                SHIP_TURN_SPEED.1
                    * ship_state.sails.as_rotation_speed()
                    * TAU
                    * time.delta_seconds(),
            );
        }

        // Adjust camera for mouse position
        let mut camera_input_offset = Vec3::ZERO;
        for mouse in mouse.iter() {
            camera_input_offset =
                Vec3::new(mouse.delta.x, mouse.delta.y, 0.0) * CAMERA_SPEED * time.delta_seconds();
        }

        let ship_offset = Vec3::ZERO
            + ship_state.sails.as_forward_speed(ship.forward().clone())
                * SHIP_SPEED
                * time.delta_seconds();

        // Update player position based on input
        ship.translation += ship_offset;

        // Update camera position based on ship position and mouse inputs
        camera.translation += camera_input_offset;

        // Apply restrictions to ensure the camera is within its bounds, relative to player position
        camera.translation = Vec3::new(
            camera.translation.x.clamp(
                ship.local_x().x - CAMERA_MAX_PAN,
                ship.local_x().x + CAMERA_MAX_PAN,
            ),
            // NOTE: Not relative to player position. This shouldn't cause issues, unless the waves are huge.
            camera.translation.y.clamp(6.0, CAMERA_MAX_HEIGHT),
            camera.translation.z,
        );

        // Have the camera look at a point relative to the parent (ship) component
        camera.look_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y);
    }
}
