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
        println!("*** UPDATING WORLD ***");

        // Update all fireflies
        let num_fireflies = self.num_entities();
        for i in 0..num_fireflies {
            // call each firefly's update function
            let mut cur_firefly = self.firefly_swarm[i].clone();
            cur_firefly.update(self as &mut World<Color>);
            self.firefly_swarm[i] = cur_firefly;
        }

        // Remove dead fireflies
        self.firefly_swarm.retain(|ref firefly| firefly.lifetime != 0);

        let num_fireflies = self.num_entities();
        // Compare remaining fireflies
        for i in 0..num_fireflies {
            for j in (i+1)..num_fireflies{
                // skip comparison to self
                if i == j {
                    continue;
                }
                // check for distances
                if self.get_dist(&self.firefly_swarm[i],
                                 &self.firefly_swarm[j]) < Firefly::SIGHT_RANGE {
                    println!("Close with {:?} and {:?}", 
                             self.firefly_swarm[i].pos, 
                             self.firefly_swarm[j].pos);
                }
            }
        }
        



        // Iterate through all fireflies in a specifc range,
        // average color
        // determine which firefles are near the current iteration
        // of the firefly and then average the message, then pass the message

        // if a firefly is in sync for a long enough time, add a new firefly
        // TODO: how to implement:
        // if fireflies flash at the same time, then they move closer
        // if they are within a close enough radius, birth new firefly
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

    // death of some entity
    fn remove_entity(&mut self, idx: usize) {
        self.firefly_swarm.swap_remove(idx);
    }

    // calculates Euclidean distance between two fireflys in n dimensional space
    fn get_dist(&self, firefly_a: &Firefly, firefly_b: &Firefly) -> f32 {
        // Iterate over coordinates in firefly a
        firefly_a.pos
            .iter()
            // Compare to coordinates in firefly b
            .zip(firefly_b.pos.iter())
            // Square all the differences
            .map(|(a_coord, b_coord)| (a_coord - b_coord).powi(2))
            // Sum the squares
            .sum::<f32>()
            // Take the square root
            .sqrt()
    }

    //This outputs the midpoint between two fireflies.
    fn calc_midpoint(&mut self, ff1:&Firefly, ff2:&Firefly) -> Vec<f32> {
        //Iterate over the coordinates in firefly 1.
        ff1
            .pos
            .iter()
            //Make tuples of coordinate components.
            .zip(ff2.pos.iter())
            //Find mid-point with respect to each component.
            .map(|(ff1_coord, ff2_coord)| {(ff1_coord + ff2_coord)/2_f32})
            //Yield new vector of the mid-points.
            .collect()
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
        Color {
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

#[derive(Debug, Clone)]
struct Firefly {
    pos: Vec<f32>,
    color: Color,            // RGB
    flash_cooldown: u32,     // initial flash cooldown
    cur_flash_cooldown: u32, // number of ticks to wait before next flash
    flash_rate: u32,         // the amt by which flash cooldown decreases
    lifetime: u32,           // the number of ticks a firefly lives for
    reproduction_range: f32, // for far a firefly must be to reproduce
}

impl Firefly {
    // associated const for sight range
    const SIGHT_RANGE: f32 = 5_f32;


    // constructor
    fn new(num_dimensions: usize) -> Self {
        Firefly {
            pos: Vec::with_capacity(num_dimensions),
            color: Color::new(num_dimensions),
            flash_cooldown: 100,     // TODO: placeholder
            cur_flash_cooldown: 100, // TODO: placeholder
            flash_rate: 1,           // TODO: placeholder
            lifetime: 500,           // TODO: placeholder
            reproduction_range: 5.0, // TODO: placeholder
        }
    }

    // construct at position
    fn new_at(pos: Vec<f32>) -> Self {
        Firefly {
            pos: pos.clone(),
            color: Color::new(pos.len()),
            flash_cooldown: 100,     // TODO: placeholder
            cur_flash_cooldown: 100, // TODO: placeholder
            flash_rate: 1,           // TODO: placeholder
            lifetime: 500,           // TODO: placeholder
            reproduction_range: 5.0, // TODO: placeholder
        }
    }

    //This outputs a unit vector which points from self to other.
    fn unit_step(&mut self, other:&Firefly, dist:f32) {
        self.pos = self
                       .pos
                       .iter()
                       //Collect respective position components.
                       .zip(other.pos.iter())
                       //Yield new component, scaled appropriately.
                       .map(|(p1_i, p2_i)| {(p2_i - p1_i)/dist})
                       //Yield new vector.
                       .collect();
    }

}

/// Fireflies communicate with lights, represented in the
/// tuple (RGB)
impl Entity<Color> for Firefly {
    // todo: receive message, send message,
    fn update(&mut self, _world: &World<Color>) {
        // Sanity check to make sure we dont update dead fireflies
        if self.lifetime == 0 {
            // grave of the fireflies
            // lol steven 
            return;
        }
        // Lose some life
        self.lifetime -= 1;

        // Tick down flash cooldown
        self.cur_flash_cooldown -= self.flash_rate;
        // At end of cooldown
        if self.cur_flash_cooldown == 0 {
            // Reset cooldown
            self.cur_flash_cooldown = self.flash_cooldown;
            // TODO: flash
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
mod test {
    use super::*;
    #[test]
    fn test_world_update() {
        let mut world = FireflyWorld {
            firefly_swarm: Vec::new(),
        };

        // add some test fireflies
        world.add_entity(Firefly::new_at(vec![5_f32,12_f32]));
        world.add_entity(Firefly::new_at(vec![0_f32,0_f32]));
        world.add_entity(Firefly::new_at(vec![0_f32,1_f32]));
        world.add_entity(Firefly::new_at(vec![7_f32,10_f32]));

        // set different lifetimes for each firefly
        world.firefly_swarm[0].lifetime = 2;
        world.firefly_swarm[1].lifetime = 3;
        world.firefly_swarm[2].lifetime = 4;
        world.firefly_swarm[3].lifetime = 11;

        // check updates for death
        assert_eq!(world.num_entities(), 4);
        world.update();
        assert_eq!(world.num_entities(), 4);
        world.update();
        assert_eq!(world.num_entities(), 3);
        world.update();
        assert_eq!(world.num_entities(), 2);
        world.update();
        assert_eq!(world.num_entities(), 1);
        for _ in 0..6 {
            world.update();
        }
        assert_eq!(world.num_entities(), 1);
        world.update();
        assert_eq!(world.num_entities(), 0);
    }

    #[test]
    fn test_get_dist() {
        let world = FireflyWorld {
            firefly_swarm: Vec::new(),
        };

        let mut a = Firefly::new(2);
        let mut b = Firefly::new(2);
        a.pos = vec![3.0, 4.0];
        b.pos = vec![0.0, 0.0];

        assert_eq!(world.get_dist(&a, &b), 5.0);
    }


    #[test]
    fn test_unit_step(){
        let world = FireflyWorld {
            firefly_swarm: Vec::new(),
        };

        let mut a = Firefly::new(2);
        let mut b = Firefly::new(2);
        a.pos.push(0.0);
        a.pos.push(0.0);
        b.pos.push(1.0);
        b.pos.push(2.0);
        let d = world.get_dist(&a, &b);
        a.unit_step(&b, d);
        assert_eq!(a.pos, vec![1_f32/5_f32.sqrt(), 2_f32/5_f32.sqrt()]);
    }

    #[test]
    fn test_midpoint(){
        let mut world = FireflyWorld {
            firefly_swarm: Vec::new(),
        };

        let mut a = Firefly::new(3);
        let mut b = Firefly::new(3);
        a.pos.push(3.0);
        a.pos.push(4.0);
        a.pos.push(5.0);
        b.pos.push(0.0);
        b.pos.push(0.0);
        b.pos.push(0.0);
        let mid = world.calc_midpoint(&a, &b);
        assert_eq!(mid, vec![1.5_f32, 2_f32, 2.5_f32]);
    }
}
