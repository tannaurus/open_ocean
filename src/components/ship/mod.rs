use super::{health::Health, id::Name};
use bevy::prelude::*;
use cannons::Cannons;

mod camera;
mod cannons;
pub mod enemy;
pub mod player;
mod sails;

#[derive(Bundle, Default)]
pub struct ShipBundle {
    name: Name,
    spatial_bundle: SpatialBundle,
    ship: Ship,
}

#[derive(Component, Default)]
pub struct Ship {
    sails: sails::SailState,
    health: Health,
    cannons: Cannons,
}

#[derive(PartialEq, Clone)]
pub enum ShipMarker {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct PlayerShip;

#[derive(Component)]
pub struct EnemyShip;

pub fn spawn_ship(
    marker: ShipMarker,
    name: &'static str,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ship_handle = asset_server.load("models/pirate_ship/dutch_ship_large_01_1k.gltf#Scene0");

    let mut ship = commands.spawn(ShipBundle {
        name: Name::new(name),
        ..default()
    });

    match marker {
        ShipMarker::Player => ship.insert(PlayerShip),
        ShipMarker::Enemy => ship.insert(EnemyShip),
    };

    ship.with_children(|parent| {
        parent.spawn(SceneBundle {
            scene: ship_handle.clone(),
            // Rotate ship model to line up with rotation axis.
            transform: Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
            ..Default::default()
        });
    });

    if marker == ShipMarker::Player {
        ship.with_children(|parent| {
            // spawn ship camera
            parent.spawn(camera::ShipCameraBundle::default());
        });
    }
}
