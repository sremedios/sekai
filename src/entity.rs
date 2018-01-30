/// Defines an entity within a world
pub trait Entity {
    /// Updates the entity based on the world around it
    pub fn update(&mut self, world: World);
}
