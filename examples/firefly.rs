/*
 * Synchronize flashing of a firefly swarm
 *
 * Fireflies are stored as a swarm in FireflyWorld.
 * The whole swarm is a hashmap, mapping an ID to the firefly object.
 * This allows us to access specific firefly members during the recieve
 * command.
 *
 * The traditional Firefly Algorithm is an optimization metaheuristic which
 * optimizes over all fireflies, not those just in the sight range, against some
 * cost or evaluation function.
 *
 * We propose using a sight range to do synchronization and movement instead of
 * a straight optimization.
 *
 */
extern crate sekai;
use sekai::world::World;
use sekai::entity::Entity;
use std::collections::HashMap;


#[derive(Debug)]
struct FireflyWorld {
    cur_id: u32, // used to track ID of each firefly in the swarm
    firefly_swarm: HashMap<u32, Firefly>,
}
impl World<Color> for FireflyWorld {
    // todo: figure out if a firefly can see another firefly
    fn update(&mut self) {
        // Iterate through all fireflies in a specifc range,
        // average color
        // determine which firefles are near the current iteration
        // of the firefly and then average the message, then pass the message
        /*
        for (id, firefly) in &mut self.firefly_swarm {
            println!("Updating firefly {}", id);
        }*/
	/* to be implemented 
	for each  firefly 
	loop through the rest of the fire flys 
	check all x,y cordinates and calculate the coordinate
	*/  	
    }

    // returns the number of fireflies in the swarm
    fn num_entities(&self) -> usize {
        self.firefly_swarm.len()
    }

    // calls receive message on every firefly
    fn receive_message(&mut self, message: Color) {
        for (_, firefly) in &mut self.firefly_swarm {
            firefly.receive_message(message.clone());
        }
    }
}

impl FireflyWorld {
    // add a new firefly
    fn add_entity(&mut self, firefly: Firefly) {
        self.firefly_swarm.insert(self.cur_id, firefly);
        self.cur_id += 1;
    }
}

#[derive(Clone, Debug)]
struct Color {
    red: f32,
    green: f32,
    blue: f32,
}
impl std::ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

#[derive(Debug)]
struct Firefly {
    x: f32,
    y: f32, // 2D world
    color: Color, // RGB
    flash_cooldown: u8, // initial flash cooldown
    cur_flash_cooldown: u8, // number of ticks to wait before next flash
    flash_rate: u8, // the amt by which flash cooldown decreases
}
/// Fireflies communicate with lights, represented in the
/// tuple (RGB)
impl Entity<Color> for Firefly {
    // todo: receive message, send message,
    fn update(&mut self, world: &World<Color>) {
        self.cur_flash_cooldown -= self.flash_rate;
        if self.cur_flash_cooldown == 0 {
            self.cur_flash_cooldown = self.flash_cooldown;
        }
    }
    fn receive_message(&mut self, message: Color) {
        // If a firefly sees some color, it must by some logic

        // Placeholder logic for now
        // for now, alpha will be some scalar adjustment
        let alpha: f32 = 1e-2;
        // If all message lights that were received were averaged by the world:
        // Scale the averaged message by some alpha step size
        self.color = message * alpha;
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
    let mut world = FireflyWorld {
        cur_id: 0,
        firefly_swarm: HashMap::new(),
    };

    for _ in 0..10 {
        world.add_entity(Firefly {
            x: 0_f32,
            y: 0_f32,
            color: Color {
                red: 100_f32,
                green: 100_f32,
                blue: 100_f32,
            },
            flash_cooldown: 30,
            cur_flash_cooldown: 30,
            flash_rate: 1,
        });
        world.update();
    }
    println!("{:?}", world.firefly_swarm);
    assert_eq!(world.firefly_swarm.len(), 10);
}
