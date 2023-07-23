use bevy::{app::AppExit, asset::ChangeWatcher, prelude::*};
use bevy_atmosphere::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_water::{ImageUtilsPlugin, WaterPlugin, WaterSettings};
use std::time::Duration;

mod components;
use components::{id, ship};
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Tell the asset server to watch for asset changes on disk:
            watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
            ..default()
        }))
        // Physics plugins
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // Water plugins
        .add_plugins(WaterPlugin)
        .add_plugins(ImageUtilsPlugin)
        .add_plugins(AtmospherePlugin)
        .insert_resource(WaterSettings {
            height: 1.0,
            amplitude: 2.0,
            ..Default::default()
        })
        // Core game plugins
        .add_state::<ui::MenuState>()
        .add_plugins(Ui)
        .add_plugins(GameMechanics)
        .run();
}

pub struct GameMechanics;
impl Plugin for GameMechanics {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, lights)
            .add_systems(Startup, world)
            .add_systems(Startup, ship::Systems::spawn_ship)
            .add_systems(
                Update,
                ship::Systems::movement.run_if(state_exists_and_equals(ui::MenuState::Ship)),
            )
            .add_systems(
                Update,
                ship::Systems::camera.run_if(state_exists_and_equals(ui::MenuState::Ship)),
            )
            .add_systems(
                Update,
                ship::Systems::cannons.run_if(state_exists_and_equals(ui::MenuState::Ship)),
            );
    }
}

pub struct Ui;
impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_pause_menu);
    }
}

// fn quit(keyboard: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
//     if keyboard.just_pressed(KeyCode::Escape) {
//         exit.send(AppExit);
//     }
// }

fn toggle_pause_menu(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    game_state: Res<State<ui::MenuState>>,
    mut next_game_state: ResMut<NextState<ui::MenuState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        let updated_state = ui::MenuState::toggle_pause(game_state.get());
        match updated_state {
            ui::MenuState::Pause => {
                commands.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                });
            }
            _ => {}
        }
        next_game_state.set(updated_state);
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
