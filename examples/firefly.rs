/*
 *
 *Synchronize flashing of a firefly swarm
 *
 * Fireflies are stored as a swarm in FireflyWorld.
 * The whole swarm is a vector, mapping an index to the firefly object.
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

#[derive(Debug)]
struct FireflyWorld {
    firefly_swarm: Vec<Firefly>,
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

        // if a firefly is in sync for a long enough time, add a new firefly
        // TODO: how to implement:
        // if fireflies flash at the same time, then they move closer
        // if they are within a close enough radius, birth new firefly 
        
        // if a firefly's life is <= 0, remove it
        // TODO: Execute this in the checking loop
        
    }
    // returns the number of fireflies in the swarm
    fn num_entities(&self) -> usize {
        self.firefly_swarm.len()
    }

    // calls receive message on every firefly
    fn receive_message(&mut self, message: Color) {
        for firefly in &mut self.firefly_swarm {
            firefly.receive_message(message.clone());
        }
    }
}
impl FireflyWorld {
    // birth of new entity
    fn add_entity(&mut self, firefly: Firefly) {
        self.firefly_swarm.push(firefly);
    }
    // calculates Euclidean distance between two fireflys in n dimensional space
    fn get_dist(&mut self, firefly_a: &Firefly, firefly_b: &Firefly) -> f32 {
        let mut sum = 0.0f32;
        for i in 0..firefly_a.pos.len() {
            sum += (firefly_a.pos[i] - firefly_b.pos[i]).powi(2)
        };
        return sum.sqrt();
    }

    // death of some entity
    fn remove_entity(&mut self, idx: usize) {
        self.firefly_swarm.swap_remove(idx);
    }
}

#[derive(Clone, Debug)]
struct Color {
    red: f32,
    green: f32,
    blue: f32,
    pos: Vec<f32>,
}

impl Color {
    fn new(num_dimensions: usize) -> Self {
        Color{
            red: 50_f32,
            green: 50_f32,
            blue: 50_f32,
            pos: Vec::with_capacity(num_dimensions),
        }
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
            pos: self.pos,
        }
    }
}

#[derive(Debug)]
struct Firefly {
    pos: Vec<f32>,
    color: Color, // RGB
    flash_cooldown: u8, // initial flash cooldown
    cur_flash_cooldown: u8, // number of ticks to wait before next flash
    flash_rate: u8, // the amt by which flash cooldown decreases
    lifetime: u8, // the number of ticks a firefly lives for
    sight_range: f32, // how far a firefly can see, radius
    reproduction_range: f32, // for far a firefly must be to reproduce
}

impl Firefly {
    // constructor
    fn new(num_dimensions: usize) -> Self {
        Firefly {
            pos: Vec::with_capacity(num_dimensions),
            color: Color::new(num_dimensions),
            flash_cooldown: 100_u8, // TODO: placeholder
            cur_flash_cooldown: 100_u8, // TODO: placeholder
            flash_rate: 1_u8, // TODO: placeholder
            lifetime: 500_u8, // TODO: placeholder
            sight_range: 50_f32, // TODO: placeholder
            reproduction_range: 5_f32, // TODO: placeholder
        }
    }

    //This outputs a unit vector which points from self to other.
    fn unit_step(&mut self, other:&Firefly, dist:f32) {
        let mut newPos = Vec::with_capacity((self.pos).len());
        for i in 0..(self.pos).len()
        {
            newPos.push((other.pos[i] - self.pos[i])/dist);
        }
        self.pos = newPos;

    }

}

/// Fireflies communicate with lights, represented in the
/// tuple (RGB)
impl Entity<Color> for Firefly {

    // todo: receive message, send message,
    fn update(&mut self, world: &World<Color>) {
        // tick down life
        self.lifetime -= 1;

        // tick down flash cooldown
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


        // TODO: update position based on the message
    }
}

fn main() {
    println!("This is the main function");
}
#[cfg(test)]
#[test]
fn test_world_update() {
    let mut world = FireflyWorld {
        firefly_swarm: Vec::new(),
    };

    for _ in 0..10 {
        world.add_entity(Firefly::new(2));
        world.update();
    }
    println!("{:?}", world.firefly_swarm);
    assert_eq!(world.firefly_swarm.len(), 10);
}

#[cfg(test)]
#[test]
fn test_get_dist(){
    let mut world = FireflyWorld {
        firefly_swarm: Vec::new(),
    };

    let mut a = Firefly::new(2);
    let mut b = Firefly::new(2);
    a.pos.push(3.0);
    a.pos.push(4.0);
    b.pos.push(0.0);
    b.pos.push(0.0);
    assert_eq!(world.get_dist(&a,&b), 5.0);
}


#[cfg(test)]
#[test]
fn test_unit_step(){
    let mut world = FireflyWorld {
        firefly_swarm: Vec::new(),
    };

    let mut a = Firefly::new(2);
    let mut b = Firefly::new(2);
    a.pos.push(0.0);
    a.pos.push(0.0);
    b.pos.push(3.0);
    b.pos.push(4.0);
    let d = world.get_dist(&a, &b);
    a.unit_step(&b, d);
    assert_eq!(a.pos, vec![3_f32/5_f32, 4_f32/5_f32]);
}
