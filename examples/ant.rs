/*
 * Ant search via stigmergy
 *
 */
extern crate sekai;
use sekai::world::World;
use sekai::entity::Entity;

#[derive(Debug)]
struct AntWorld {
    food_locations: Vec<Food>,
    pheromone_trail: Vec<Pheromone>,
    ant_swarm: Vec<Ant>,
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
        for ant in &mut self.ant_swarm {
            ant.receive_message(message.clone());
        }
    }
}

impl AntWorld {
    // add a new ant
    fn add_entity(&mut self, ant: Ant) {
        self.ant_swarm.push(ant);
    }

    fn new() -> Self {
        AntWorld{
            food_locations: Vec::new(),
            pheromone_trail: Vec::new(),
            ant_swarm: Vec::new(),
        }
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
        //message.update();
    }
}
impl Ant {
    fn new() -> Self {
        Ant{
            x: 0_f32,
            y: 0_f32,
          stride: 1_f32,
          pheromone_sense_threshold: 2_u32,
        }
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
    lifetime: f32,
    decay: f32,
    sensitivity: f32, // how far away an ant can be to sense it
}
impl Pheromone {
    fn update(&mut self) {
        self.lifetime -= self.decay;
    }
}

fn main() {
    println!("This is the main function");
}

#[test]
fn test_world_update() {
    let mut world = AntWorld::new();
    for _ in 0..10 {
        world.add_entity(Ant::new());
    }
    println!("{:#?}", world.ant_swarm);
    assert_eq!(world.num_entities(), 10);
}
