/*
 * Ant search via stigmergy
 *
 */
extern crate sekai;
use sekai::world::World;
use sekai::entity::Entity;
use std::collections::HashMap;

#[derive(Debug)]
struct AntWorld {
    cur_id: u32, // used to track ID of each ant in the swarm
    food_locations: Vec<Food>,
    pheromone_trail: Vec<Pheromone>,
    ant_swarm: HashMap<u32, Ant>,
    ant_hive: (f32, f32), // where all ants want to go c:
}
impl World<Pheromone> for AntWorld {
    // todo: figure out if a ant can see another ant
    fn update(&mut self) {
        // TODO: call update on each pheromone trail item
    }

    // returns the number of ants in the swarm
    fn num_entities(&self) -> usize {
        self.ant_swarm.len()
    }

    // calls receive message on every ant
    fn receive_message(&mut self, message: Pheromone) {
        for (_, ant) in &mut self.ant_swarm {
            ant.receive_message(message.clone());
        }
    }
}

impl AntWorld {
    // add a new ant
    fn add_entity(&mut self, ant: Ant) {
        self.ant_swarm.insert(self.cur_id, ant);
        self.cur_id += 1;
    }
}

#[derive(Debug)]
struct Ant {
    x: f32,
    y: f32, // 2D world
    stride: f32, // how much the ant travels per tick
    pheromone_sense_threshold: u32, // minimum value needed to follow pheromone trail
}
impl Entity<Pheromone> for Ant {
    // todo: receive message, send message,
    fn update(&mut self, world: &World<Pheromone>) {
        
    }
    fn receive_message(&mut self, message: Pheromone) {
        // TODO increase current position by some step size in the
        // direction of the message.  Maybe the world should decide where the
        // next message to send is?  IE: the world says in which direction the
        // ant should step
        self.x += message.x;
        self.y += message.y;
    }
}

#[derive(Debug)]
struct Food {
    // Food has a location and some limited resource count
    x: f32,
    y: f32,
    resource: u32,
}

#[derive(Debug, Clone)]
struct Pheromone {
    // need to have a location and a lifetime
    // if an ant walks over an already-existing pheromone, it
    // strengthens it by resetting the lifetime timer
    x: f32,
    y: f32,
    strength: u32,
    decay: u32,
}
impl Pheromone {
    fn update(&mut self) {
        self.strength -= self.decay;
    }
}

fn main() {
    println!("This is the main function");
}

#[test]
fn test_world_update() {
    let mut world = AntWorld {
        cur_id: 0,
        food_locations: vec![
            Food{x: 5_f32, 
                y: 5_f32, 
                resource: 50,
            },
            Food{x: -5_f32, 
                y: -5_f32, 
                resource: 50,
            },
        ],
        pheromone_trail: Vec::new(),
        ant_swarm: HashMap::new(),
        ant_hive: (0_f32, 0_f32),
    };
    for i in 0..10 {
        world.add_entity(Ant {
            x: i as f32,
            y: i as f32,
            stride: 1_f32, // placeholder
            pheromone_sense_threshold: 2, // placeholder
        });
    }
    println!("{:#?}", world.ant_swarm);
    assert_eq!(world.num_entities(), 10);
}
