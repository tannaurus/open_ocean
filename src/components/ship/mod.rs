use super::{collider_group::AsCollisionGroups, health::Health, id::Name};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    AdditionalMassProperties, Collider, CollisionGroups, Damping, LockedAxes, RigidBody, Velocity,
};
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
    rigidbody: RigidBody,
    locked_axis: LockedAxes,
    mass: AdditionalMassProperties,
    velocity: Velocity,
    damping: Damping,
}

#[derive(Component, Default)]
pub struct Ship {
    sails: sails::SailState,
    health: Health,
    cannons: Cannons,
}

#[derive(PartialEq, Clone, Component)]
pub enum ShipMarker {
    Player,
    Enemy,
}

impl Default for ShipMarker {
    fn default() -> Self {
        Self::Player
    }
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
        rigidbody: RigidBody::Dynamic,
        locked_axis: LockedAxes::TRANSLATION_LOCKED_Y
            | LockedAxes::ROTATION_LOCKED_X
            | LockedAxes::ROTATION_LOCKED_Z,
        mass: AdditionalMassProperties::Mass(2000.0),
        damping: Damping {
            linear_damping: 100.0,
            angular_damping: 20.0,
        },
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
            // Spawn ship camera
            parent.spawn(camera::ShipCameraBundle::default());
        });
    }
}
