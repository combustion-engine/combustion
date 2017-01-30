//! Traits for "named" types. That is, they have a `name: String` field

/// Trait for types with a `name` field
pub trait Named {
    /// Get the name
    fn name(&self) -> &String;
    /// Set the name
    fn set_name(&mut self, name: String);
}

/// Since there are many named types, define a single trait for that
pub trait DefaultName {
    /// Returns the default name for the type
    fn default_name() -> String;
}