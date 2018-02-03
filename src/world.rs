/// Defines an object representing a world
pub trait World<M> {
    /// Updates the world 1 tick
    fn update(&mut self);
    /// Gets the number of entities in the world
    fn num_entities(&self) -> usize;
    /// Handles a message sent by an entity
    /// # Arguments
<<<<<<< HEAD
    /// * `message` - The message being received from an entity 
=======
    /// * `message` - The message being received from an entity
>>>>>>> fbdaa6c88a03e4b118e536d596e1a0f376280dbe
    fn receive_message(&mut self, message: M);
}
