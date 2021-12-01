// Declare the two files for pet and pets
mod pets;
mod pet;

// Not needed for most cases but re-exporting these at top level. 
pub use pets::Pets;
pub use pet::Pet;