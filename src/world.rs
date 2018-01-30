/// Defines an object representing a world
pub trait World<M> {
    /// Updates the world 1 tick
    fn update(&mut self);
    /// Gets the number of entities in the world
    fn num_entities(&self) -> usize;
    /// Handles a message sent by an entity
    fn receive_message(&mut self, message: M);
}
