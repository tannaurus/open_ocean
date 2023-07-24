use super::{collider_group::AsCollisionGroups, health::Health, id::Name};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, CollisionGroups};
use cannons::Cannons;

mod camera;
pub mod cannons;
pub mod enemy;
pub mod player;
mod sails;

#[derive(Bundle, Default)]
pub struct ShipBundle {
    name: Name,
    spatial_bundle: SpatialBundle,
    collider: Collider,
    ship: Ship,
    collision_group: CollisionGroups,
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
    location: Vec3,
    commands: &mut Commands,
    ship_handle: &Handle<Scene>,
    ship_collider: Collider,
) {
    let mut ship = commands.spawn(ShipBundle {
        name: Name::new(name),
        spatial_bundle: SpatialBundle::from_transform(Transform::from_translation(location)),
        collider: ship_collider,
        collision_group: marker.as_collision_groups(),
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
