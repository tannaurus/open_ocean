use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    max_health: f32,
    starting_health: f32,
    current_health: f32,
    dead: bool,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            max_health: 100.0,
            starting_health: 100.0,
            current_health: 100.0,
            dead: false,
        }
    }
}

impl Health {
    pub fn take_damage(&mut self, amount: f32) {
        if self.dead {
            return;
        }

        if self.current_health - amount <= 0.0 {
            self.dead = true;
            self.current_health = 0.0;
            println!("I'm dead!");
            return;
        }

        self.current_health -= amount;
    }

    pub fn heal(&mut self, amount: f32) {
        if self.dead {
            return;
        }

        if self.current_health + amount >= self.max_health {
            self.current_health = self.max_health;
            return;
        }

        self.current_health += amount;
    }
}
