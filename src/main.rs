use bevy::{app::AppExit, asset::ChangeWatcher, prelude::*};
use bevy_water::{ImageUtilsPlugin, WaterPlugin, WaterSettings};
use std::time::Duration;

mod components;
use components::{id, ship};

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
        .insert_resource(WaterSettings {
            height: 1.0,
            amplitude: 2.0,
            ..Default::default()
        })
        // core game plugins
        .add_plugins(Game)
        .run();
}

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, lights)
            .add_systems(Startup, world)
            .add_systems(Startup, ship::Systems::spawn_ship)
            .add_systems(Update, ship::Systems::ship_controller)
            .add_systems(Update, quit);
    }
}

fn quit(keyboard: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
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
