/*
 * Synchronize flashing of a firefly swarm
 *
 * Fireflies are stored as a swarm in FireflyWorld.
 * The whole swarm is a hashmap, mapping an ID to the firefly object.
 * This allows us to access specific firefly members during the recieve
 * command.
 *
 */
extern crate sekai;
use sekai::world::World;
use sekai::entity::Entity;
use std::collections::HashMap;

struct FireflyWorld {
    cur_ID: u32, // used to track ID of each firefly in the swarm
    firefly_swarm: HashMap<u16, Firefly>,
}
impl World<(u8, u8, u8)> for FireflyWorld {
    // todo: figure out if a firefly can see another firefly
    fn update(&mut self) {
        // Iterate through all fireflies in a specifc range,
        // average color
        for (id, firefly) in &mut self.firefly_swarm {
            println!("Updating firefly {}", id);
        }
    }

    // returns the number of fireflies in the swarm
    fn num_entities(&self) -> usize {
        self.firefly_swarm.len()
    }

    // calls receive message on every firefly
    fn receive_message(&mut self, message: (u8, u8, u8)) {
        for (id, firefly) in &mut self.firefly_swarm {
            firefly.receive_message(message);
        }
    }
}

struct Firefly {
    x: f32,
    y: f32, // 2D world
    id: u16, // determined based on the world's ID tracker
    color: (u8, u8, u8), // RGB
    flash_cooldown: u8, // initial flash cooldown
    cur_flash_cooldown: u8, // number of ticks to wait before next flash
    flash_rate: u8, // the amt by which flash cooldown decreases
}
/// Fireflies communicate with lights, represented in the
/// tuple (RGB)
impl Entity<(u8, u8, u8)> for Firefly {
    // todo: receive message, send message,
    fn update(&mut self, world: &World<(u8, u8, u8)>) {
        self.cur_flash_cooldown -= self.flash_rate;
        if self.cur_flash_cooldown == 0 {
            self.cur_flash_cooldown = self.flash_cooldown;
        }
    }
    fn receive_message(&mut self, message: (u8, u8, u8)) {
        // If a firefly sees some color, it must by some logic
        // update its own flash_rate and color

        // Placeholder logic for now
        // If all message lights that were received were averaged:
        self.color = message;
        // If received, reset cur_flash_cooldown
        self.cur_flash_cooldown = self.flash_cooldown;
        // how to update flash rate?  should this even be parameterized?


    }
}

fn main() {
    println!("This is the main function");
}

#[test]
fn test_world_update() {
    world = FireflyWorld { cur_ID: 0 };

}
