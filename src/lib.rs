pub mod cli;
pub mod commands;
pub mod objects;
pub mod store;


pub use cli::{Cli, Commands};
pub use commands::{add, diff, init, cat};
pub use objects::{blob, tree, types::TreeEntry};
pub use store::{
    index::{add_to_index, read_index, write_index},
    object_store::{read_object, save_to_objects},
    repo::find_root,
};