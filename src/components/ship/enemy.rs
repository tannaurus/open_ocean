use super::{spawn_ship, ShipMarker};
use bevy::prelude::*;

pub struct Systems;

impl Systems {
    pub fn spawn_ship(commands: Commands, asset_server: Res<AssetServer>) {
        spawn_ship(ShipMarker::Enemy, "S.S Bath Time", commands, asset_server);
    }
}
