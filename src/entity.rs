use world::World;

/// Defines an entity within a world
/// # Arguments
/// * `M` - The type to use for messages between entities
/// # Example
/// ```
/// struct Cat {
///     hunger: i64,
/// }
/// impl Entity<String> for Cat {
///     /// Every tick, the cat gets more hungry
///     /// If the cat gets too hungry, it meows
///     fn update(&mut self, world: &World) {
///         self.hunger += 1;
///         if self.hunger >= 100 {
///             self.send_all("MEOW".into());
///         }
///     }
///     /// Cats send messages to stdout
///     fn send_all(&self, message: String, world: &World) {
///         println!("{}", message);
///     }
///     /// Cats ignore any incoming messages
///     fn receive(&mut  self, message: T) {}
/// }
/// ```
pub trait Entity<M> {
    /// Updates the entity based on the world around it
    /// # Arguments
    /// * `world` - the world the entity exists in
    fn update(&mut self, world: &World<M>);
    /// Sends a message to every entity in the world
    /// # Arguments
    /// * `message` - The message being sent to all other entities
    /// * `world` - The world the entity exists in
    fn send_all(&self, message: M, world: &World<M>);
    /// Handler for receiving a message
    /// # Arguments
    /// * `message` - The message to receive
    fn receive(&mut self, message: M);
}
