use super::ship;
use bevy_rapier3d::prelude::*;

pub trait AsCollisionGroups {
    fn as_collision_groups(&self) -> CollisionGroups;
}

const PLAYER_SHIP: Group = Group::GROUP_1;
const PLAYER_CANNON: Group = Group::GROUP_2;
const ENEMY_SHIP: Group = Group::GROUP_3;
const ENEMY_CANNON: Group = Group::GROUP_4;

impl AsCollisionGroups for ship::cannons::CannonMarker {
    fn as_collision_groups(&self) -> CollisionGroups {
        match self {
            ship::cannons::CannonMarker::Player => {
                CollisionGroups::new(PLAYER_CANNON, Group::ALL ^ PLAYER_SHIP ^ PLAYER_CANNON)
            }
            ship::cannons::CannonMarker::Enemy => {
                CollisionGroups::new(ENEMY_CANNON, Group::ALL ^ ENEMY_SHIP ^ ENEMY_CANNON)
            }
        }
    }
}

impl AsCollisionGroups for ship::ShipMarker {
    fn as_collision_groups(&self) -> CollisionGroups {
        match self {
            ship::ShipMarker::Player => CollisionGroups::new(PLAYER_SHIP, Group::ALL),
            ship::ShipMarker::Enemy => CollisionGroups::new(ENEMY_SHIP, Group::ALL),
        }
    }
}
