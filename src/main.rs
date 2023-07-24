use bevy::{asset::ChangeWatcher, prelude::*};
use bevy_atmosphere::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_water::{ImageUtilsPlugin, WaterPlugin, WaterSettings};
use std::time::Duration;

mod components;
use components::ship::{enemy, player};
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
            clarity: 0.125,
            base_color: Color::rgba(0.0, 0.75, 1.0, 0.2),
            deep_color: Color::rgba(0.0, 0.75, 1.0, 0.2),
            ..Default::default()
        })
        // Core game plugins
        .add_state::<MenuState>()
        .add_plugins(Ui)
        .add_plugins(GameMechanics)
        .run();
}

pub struct GameMechanics;
impl Plugin for GameMechanics {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, lights)
            .add_systems(Startup, world)
            .add_systems(Startup, player::Systems::spawn_ship)
            .add_systems(Startup, enemy::Systems::spawn_ship)
            .add_systems(
                Update,
                player::Systems::movement.run_if(state_exists_and_equals(MenuState::Ship)),
            )
            .add_systems(
                Update,
                player::Systems::camera.run_if(state_exists_and_equals(MenuState::Ship)),
            )
            .add_systems(
                Update,
                player::Systems::cannons.run_if(state_exists_and_equals(MenuState::Ship)),
            );
    }
}

pub struct Ui;
impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_menu_state)
            .add_systems(OnEnter(MenuState::Pause), ui::pause::render_pause_menu)
            .add_systems(OnExit(MenuState::Pause), ui::pause::close_pause_menu)
            .add_systems(
                Update,
                ui::pause::pause_menu_interactions
                    .run_if(state_exists_and_equals(MenuState::Pause)),
            );
    }
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum MenuState {
    Pause,
    Ship,
}

impl Default for MenuState {
    fn default() -> Self {
        Self::Ship
    }
}

impl MenuState {
    pub fn toggle_pause(current_state: &Self) -> Self {
        match current_state {
            Self::Pause => Self::Ship,
            _ => Self::Pause,
        }
    }
}

pub fn change_menu_state(
    keyboard: Res<Input<KeyCode>>,
    game_state: Res<State<MenuState>>,
    mut next_game_state: ResMut<NextState<MenuState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        let updated_state = MenuState::toggle_pause(game_state.get());
        println!("Updated menu state 👉 {:?}", updated_state);
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
