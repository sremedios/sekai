/// Defines an object representing a world
pub trait World {
    /// Updates the world 1 tick
    pub fn update(&mut self);
}
