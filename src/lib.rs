pub mod cli;
pub mod commands;
pub mod objects;
pub mod store;


pub use cli::{Cli, Commands};
pub use commands::{add, diff, init, cat};
pub use objects::{blob, tree, types::TreeEntry};
pub use store::{object_store::{read_object, save_to_objects}};