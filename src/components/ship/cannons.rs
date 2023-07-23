use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::time::Duration;

const CANNON_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Cannons {
    reload_speed: u64,
    left_last_launched: Duration,
    right_last_launched: Duration,
}

impl Default for Cannons {
    fn default() -> Self {
        Self {
            reload_speed: 3,
            left_last_launched: Duration::from_secs(0),
            right_last_launched: Duration::from_secs(0),
        }
    }
}

impl Cannons {
    pub fn fire(
        &mut self,
        commands: &mut Commands,
        time_elapsed: Duration,
        ship_transform: &Transform,
        direction: CannonDirection,
    ) {
        // Check if this direction's cannons are still being reloaded
        // If they have already been reloaded, mark this direction as launched.
        match direction {
            CannonDirection::Left => {
                if time_elapsed < self.left_last_launched + Duration::from_secs(self.reload_speed) {
                    println!("Reloading these cannons! 🏴‍☠️");
                    return;
                } else {
                    self.left_last_launched = time_elapsed;
                }
            }
            CannonDirection::Right => {
                if time_elapsed < self.right_last_launched + Duration::from_secs(self.reload_speed)
                {
                    println!("Reloading these cannons! 🏴‍☠️");
                    return;
                } else {
                    self.right_last_launched = time_elapsed;
                }
            }
        }
        let instance = CannonBall::instance(ship_transform, direction);
        commands.spawn(CannonBall::adjust_fire_location(instance.clone(), -10.0));
        commands.spawn(CannonBall::adjust_fire_location(instance.clone(), -5.0));
        commands.spawn(instance);
    }
}

#[derive(Bundle, Clone)]
struct CannonBall {
    rigidbody: RigidBody,
    collider: Collider,
    restitution: Restitution,
    velocity: Velocity,
    transform: TransformBundle,
    gravity: GravityScale,
}

pub enum CannonDirection {
    Left,
    Right,
}

impl CannonDirection {
    fn as_linvel(&self) -> f32 {
        match self {
            Self::Left => -1.0,
            Self::Right => 1.0,
        }
    }
}

impl CannonBall {
    fn instance(ship_transform: &Transform, direction: CannonDirection) -> Self {
        let mut transform = TransformBundle::from(ship_transform.clone());
        // Adjust launch height so they don't launch below the water line
        transform.local.translation += Vec3::new(0.0, 2.0, 0.0);
        Self {
            rigidbody: RigidBody::Dynamic,
            collider: Collider::ball(1.0),
            restitution: Restitution::coefficient(0.7),
            velocity: Velocity {
                linvel: ship_transform.local_x() * CANNON_SPEED * direction.as_linvel()
                    + (Vec3::Y * 20.0),
                angvel: Vec3::ZERO,
            },
            transform,
            gravity: GravityScale(3.0),
        }
    }

    fn adjust_fire_location(mut cannon_ball: Self, amount: f32) -> Self {
        cannon_ball.transform.local.translation += Vec3::new(0.0, 0.0, amount);
        cannon_ball
    }
}
