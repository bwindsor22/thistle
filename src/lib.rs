#[macro_use]
extern crate log;

pub mod model;
pub use model::bert::Bert;
pub use model::pooling::{Pooling, PoolingConfig};
pub use model::sentence_transformer::SentenceTransformer;
