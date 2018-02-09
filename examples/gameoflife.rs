/*
 * Game of Life turing complete test
 */

extern crate sekai;
use sekai::world::World;
use sekai::entity::Entity;
use std::cmp::{Eq,PartialEq};
type Proximity = u32;

#[derive(Debug)]
struct Board {
    // 2D discrete movement
    width: u32,
    height: u32,
    cell_swarm: Vec<Cell>,
    // game rule vector? maybe rules are the message?
    // maybe proximity is the message?
}
impl World<Proximity> for Board {
    fn update(&mut self) {
        // TODO: update the cells based on the rules
    }
    fn num_entities(&self) -> usize {
        self.cell_swarm.len()
    }
    fn receive_message(&mut self, _message: Proximity) {
        // TODO: for each cell, calculate its location based on proximity
        // and make updates based on rules
        // TODO: don't double borrow here
        let mut cur_idx = 0;
        for cell in &mut self.cell_swarm {
            if cell != &mut self.cell_swarm[cur_idx] {
                println!("Found other cell!");
            }
            cur_idx += 1;
        }

    }
}
impl Board {
    fn add_entity(&mut self, cell: Cell) {
        self.cell_swarm.push(cell);
    }
    fn remove_entity(&mut self, index: usize) {
        self.cell_swarm.swap_remove(index);
    }
    fn new() -> Self {
        Board {
            width: 10_u32,
            height: 10_u32,
            cell_swarm: Vec::new(),
        }
    }
}


#[derive(Debug)] //, Eq, PartialEq)]
struct Cell {
    // 2D discrete movement
    x: u32,
    y: u32,
}
impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Cell {}

impl Entity<Proximity> for Cell {
    fn update(&mut self, _board: &World<Proximity>) {
    }
    fn receive_message(&mut self, _message: Proximity) {

    }

}
impl Cell {
    fn new() -> Self {
        Cell { x: 0_u32, y: 0_u32,}
    }
}

fn main() {
    println!("This is the main function");
}

#[test]
fn test_game_of_life() {
    let mut board = Board::new();
    for _ in 0..10 {
        board.add_entity(Cell::new());
    }
    assert_eq!(board.num_entities(), 10);

    board.remove_entity(3);
    assert_eq!(board.num_entities(), 9);

    let test_message: Proximity = 5;
    board.receive_message(test_message);
}


