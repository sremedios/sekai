use world::World;

/// Defines an entity within a world
pub trait Entity<T> {
    /// Updates the entity based on the world around it
    fn update(&mut self, world: &World);
    /// Sends a message to every entity in the world
    /// TODO: add some kind of filter
    fn send_all(&self, message: T, world: &World);
    /// Receives a message
    fn receive(&mut self, message: T);
}
