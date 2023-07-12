use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod components;

use components::{id, player};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Game)
        .run();
}

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_physics)
            .add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Adding player_1");
    commands.spawn((player::Player, id::Name::new("player_1".to_string())));
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            subdivisions: 2,
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    // commands
    //     .spawn(Collider::cuboid(0.0, 50.0, 0.0))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    // /* Create the bouncing ball. */
    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(Collider::ball(50.0))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));

    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });
}
