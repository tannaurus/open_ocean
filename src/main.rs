use bevy::{app::AppExit, input::mouse::MouseMotion, prelude::*};

pub mod components;

use bevy_water::{ImageUtilsPlugin, WaterPlugin};
use components::{id, player};

const PLAYER_SPEED: f32 = 1.0;
const CAMERA_SPEED: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // water plugins
        .add_plugins(WaterPlugin)
        .add_plugins(ImageUtilsPlugin)
        // core game plugins
        .add_plugins(Game)
        .run();
}

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, lights)
            .add_systems(Startup, world)
            .add_systems(Startup, player)
            .add_systems(Update, player_controller)
            .add_systems(Update, quit);
    }
}

fn quit(keyboard: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        player::Player,
        id::Name::new("player_1".to_string()),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                ..Default::default()
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
    ));
    commands.spawn((
        player::PlayerCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}

fn player_controller(
    mut player: Query<&mut Transform, (With<player::Player>, Without<player::PlayerCamera>)>,
    mut camera: Query<&mut Transform, (With<player::PlayerCamera>, Without<player::Player>)>,
    mut mouse: EventReader<MouseMotion>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 0.0, -1.0);
    }
    if keyboard.pressed(KeyCode::A) {
        direction += Vec3::new(-0.5, 0.0, 0.0);
    }
    if keyboard.pressed(KeyCode::S) {
        direction += Vec3::new(0.0, 0.0, 0.5);
    }
    if keyboard.pressed(KeyCode::D) {
        direction += Vec3::new(0.5, 0.0, 0.0);
    }
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    let mut player = player.single_mut();
    let mut camera = camera.single_mut();

    // adjust camera for mouse position
    for mouse in mouse.iter() {
        camera.translation +=
            Vec3::new(mouse.delta.x, mouse.delta.y, 0.0) * CAMERA_SPEED * time.delta_seconds();

        camera.translation = Vec3::new(
            camera.translation.x.clamp(-15.0, 15.0),
            camera.translation.y.clamp(6.0, 20.0),
            camera.translation.z,
        );
    }

    // update player position based on input
    player.translation += direction * PLAYER_SPEED * time.delta_seconds();
    // update camera position based on player
    camera.translation += direction * PLAYER_SPEED * time.delta_seconds();
    camera.look_at(player.translation, Vec3::Y);
}

fn world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            subdivisions: 2,
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}

fn lights(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.8,
    });
}
