pub mod filemod;
pub mod foldermodule;
pub mod database;
pub mod model;
pub mod hnswlib;

// #![allow(dead_code, non_snake_case)]
#[cfg(feature = "blas")]
extern crate blas_src;
extern crate ndarray;
mod hash;
pub mod lsh;
pub mod dist;
mod multi_probe;
mod table {
    pub mod general;
    pub mod mem;
    pub mod sqlite;
    pub mod sqlite_mem;
}
mod constants;
mod error;

#[cfg(feature = "workspace")]
pub mod utils;
#[cfg(not(feature = "workspace"))]
mod utils;
pub use hash::VecHash;
pub use multi_probe::{QueryDirectedProbe, StepWiseProbe};
pub use table::{general::HashTables, mem::MemoryTable};
#[cfg(feature = "sqlite")]
pub use table::{sqlite::SqlTable, sqlite_mem::SqlTableMem};
pub mod data;
pub mod prelude;
pub mod stats;
