use bevy::{app::AppExit, asset::ChangeWatcher, input::mouse::MouseMotion, prelude::*};
use bevy_water::{ImageUtilsPlugin, WaterPlugin};
use std::time::Duration;

mod components;
use components::{id, player};

const PLAYER_SPEED: f32 = 6.0;
const CAMERA_SPEED: f32 = 1.0;
const CAMERA_MAX_HEIGHT: f32 = 40.0;
// Max distance camera can pan left and right
const CAMERA_MAX_PAN: (f32, f32) = (-25.0, 25.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Tell the asset server to watch for asset changes on disk:
            watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
            ..default()
        }))
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
            .add_systems(Startup, spawn_ship)
            .add_systems(Update, ship_controller)
            .add_systems(Update, quit);
    }
}

fn quit(keyboard: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn spawn_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ship_handle = asset_server.load("models/pirate_ship/dutch_ship_large_01_1k.gltf#Scene0");
    commands
        .spawn((
            player::Player,
            id::Name::new("player_1".to_string()),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    ..Default::default()
                })),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: ship_handle.clone(),
                // Rotate ship model to line up with rotation axis.
                transform: Transform::from_rotation(Quat::from_rotation_y(
                    std::f32::consts::FRAC_PI_2,
                )),
                ..Default::default()
            });
        });

    commands.spawn((
        player::PlayerCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FogSettings {
            color: Color::rgba(0.6, 0.6, 0.6, 1.0),
            falloff: FogFalloff::Exponential { density: 0.0003 },
            ..default()
        },
    ));
}

fn ship_controller(
    mut ship: Query<&mut Transform, (With<player::Player>, Without<player::PlayerCamera>)>,
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

    let mut ship = ship.single_mut();
    let mut camera = camera.single_mut();

    // adjust camera for mouse position
    for mouse in mouse.iter() {
        camera.translation +=
            Vec3::new(mouse.delta.x, mouse.delta.y, 0.0) * CAMERA_SPEED * time.delta_seconds();

        camera.translation = Vec3::new(
            camera
                .translation
                .x
                .clamp(CAMERA_MAX_PAN.0, CAMERA_MAX_PAN.1),
            camera.translation.y.clamp(6.0, CAMERA_MAX_HEIGHT),
            camera.translation.z,
        );
    }

    // update player position based on input
    ship.translation += direction * PLAYER_SPEED * time.delta_seconds();
    // update camera position based on player
    camera.translation += direction * PLAYER_SPEED * time.delta_seconds();
    camera.look_at(ship.translation, Vec3::Y);
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
        brightness: 1.5,
    });
}
