// extern crate rand;

// for logging (debug mostly, switched at compile time in cargo.toml)
// extern crate log;
// use env_logger::{Builder};

// #[macro_use]
// extern crate lazy_static;


// pub mod annhdf5;
pub mod api;
pub mod dist;
pub mod flatten;
pub mod hnsw;
pub mod hnswio;
pub mod libext;
pub mod test;

pub use hnsw::*;
pub use dist::*;
// pub use annhdf5::*;
pub use api::*;
pub use test::*;

// lazy_static! {
//     #[allow(dead_code)]
//     static ref LOG: u64 = {
//         let res = init_log();
//         res
//     };
// }

// // install a logger facility
// fn init_log() -> u64 {
//     Builder::from_default_env().init();
//     println!("\n ************** initializing logger *****************\n");    
//     return 1;
// }
