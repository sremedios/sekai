/*
 * Synchronize flashing of a firefly swarm
 *
 */
extern crate sekai;
use world::World;
use entity::Entity;

struct FireflyWorld {
}
impl World for FireflyWorld {
    // todo: figure out if a firefly can see another firefly
    fn update(&mut self) {
        // Iterate through all fireflies in a specifc range,
        // average color 
        for firefly in firefly_swarm {
            
        }
    }
}

struct Firefly {
    x: f32,
    y: f32, // 2D world
    color: (u8, u8, u8), // RGB
    flash_cooldown: u8, // initial flash cooldown
    cur_flash_cooldown: u8, // number of ticks to wait before next flash
    flash_rate: u8, // the amt by which flash cooldown decreases
}
/// Fireflies communicate with lights, represented in the
/// tuple (RGB)
impl Entity<(u8, u8, u8)> for Firefly {
    // todo: receive message, send message, 
    fn update(&mut self, world: &World) {
        self.cur_flash_cooldown -= flash_rate;
        if self.cur_flash_cooldown == 0 {
            self.cur_flash_cooldown = self.flash_cooldown;
        }
    }
    fn receive_message(&mut self, message: <(u8,u8,u8)>){
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

// let test_world: World = World::new();

#[test]
fn test_examples() {
    assert_eq!(2, 2);
}
