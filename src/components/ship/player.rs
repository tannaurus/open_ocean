use std::f32::consts::TAU;

use bevy::{input::mouse::MouseMotion, prelude::*};

use super::camera::ShipCamera;
use super::cannons::CannonDirection;
use super::{spawn_ship, PlayerShip, Ship, ShipMarker};

const SHIP_SPEED: f32 = 6.0;
// Base ship turn speed. Will be modified by the ship's velocity.
const SHIP_TURN_SPEED: (f32, f32) = (-0.05, 0.05);
const CAMERA_SPEED: f32 = 1.0;
const CAMERA_MAX_HEIGHT: f32 = 40.0;
// Max distance camera can pan left and right
const CAMERA_MAX_PAN: (f32, f32) = (-40.0, 40.0);

pub struct Systems;
impl Systems {
    pub fn spawn_ship(commands: Commands, asset_server: Res<AssetServer>) {
        spawn_ship(ShipMarker::Player, "Eleanor", commands, asset_server);
    }

    pub fn movement(
        mut ship: Query<(&mut Transform, &mut Ship), With<PlayerShip>>,
        keyboard: Res<Input<KeyCode>>,
        time: Res<Time>,
    ) {
        let (mut ship, mut ship_state) = ship.single_mut();

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

        let ship_offset = Vec3::ZERO
            + ship_state.sails.as_forward_speed(ship.forward().clone())
                * SHIP_SPEED
                * time.delta_seconds();

        // Update player position based on input
        ship.translation += ship_offset;
    }

    pub fn camera(
        mut camera: Query<&mut Transform, With<ShipCamera>>,
        mut mouse: EventReader<MouseMotion>,
        time: Res<Time>,
    ) {
        let mut camera = camera.single_mut();
        // Adjust camera for mouse position
        let mut camera_input_offset = Vec3::ZERO;
        for mouse in mouse.iter() {
            camera_input_offset =
                Vec3::new(mouse.delta.x, mouse.delta.y, 0.0) * CAMERA_SPEED * time.delta_seconds();
        }

        // Update camera position based on ship position and mouse inputs
        camera.translation += camera_input_offset;

        // Apply restrictions to ensure the camera is within its bounds, relative to player position
        camera.translation = Vec3::new(
            camera
                .translation
                .x
                .clamp(CAMERA_MAX_PAN.0, CAMERA_MAX_PAN.1),
            // NOTE: Not relative to player position. This shouldn't cause issues, unless the waves are huge.
            camera.translation.y.clamp(6.0, CAMERA_MAX_HEIGHT),
            camera.translation.z,
        );

        // Have the camera look at a point relative to the parent (ship) component
        camera.look_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y);
    }

    pub fn cannons(
        mut commands: Commands,
        mut ship: Query<(&mut Ship, &Transform), With<PlayerShip>>,
        keyboard: Res<Input<KeyCode>>,
        time: Res<Time>,
    ) {
        let (mut ship, ship_transform) = ship.single_mut();
        if keyboard.just_pressed(KeyCode::Q) {
            ship.cannons.fire(
                &mut commands,
                time.elapsed(),
                ship_transform,
                CannonDirection::Left,
            );
        }

        if keyboard.just_pressed(KeyCode::E) {
            ship.cannons.fire(
                &mut commands,
                time.elapsed(),
                ship_transform,
                CannonDirection::Right,
            );
        }
    }
}
