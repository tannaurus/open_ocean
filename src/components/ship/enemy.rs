use super::{spawn_ship, ShipMarker};
use bevy::prelude::*;

pub struct Systems;

impl Systems {
    pub fn spawn_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
        let ship_handle =
            asset_server.load("models/pirate_ship/dutch_ship_large_01_1k.gltf#Scene0");
        spawn_ship(
            ShipMarker::Enemy,
            "S.S Bath Time",
            Vec3::new(-50.0, 0.0, 0.0),
            &mut commands,
            &ship_handle,
        );
        spawn_ship(
            ShipMarker::Enemy,
            "S.S Bath Time",
            Vec3::new(50.0, 0.0, 0.0),
            &mut commands,
            &ship_handle,
        );
        spawn_ship(
            ShipMarker::Enemy,
            "S.S Bath Time",
            Vec3::new(-50.0, 0.0, -50.0),
            &mut commands,
            &ship_handle,
        );
        spawn_ship(
            ShipMarker::Enemy,
            "S.S Bath Time",
            Vec3::new(50.0, 0.0, 50.0),
            &mut commands,
            &ship_handle,
        );
    }
}
