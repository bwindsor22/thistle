use uuid::Uuid;
use crate::database::cosine_db::CosineDB;
use crate::database::euclidean_db::EuclideanDB;
use crate::database::hnsw_euclidean_db::HnswEuclideanDB;
use crate::database::hnsw_cosine_db::HnswCosineDB;
use crate::hnswlib::*;

pub fn database_module_uuid() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Debug)]
pub struct Doc {
    pub text: String,
    pub embedding: Vec<f64>,
    pub score: f64,
}

pub enum DB {
    CosineDB(CosineDB),
    EuclideanDB(EuclideanDB),
    HnswEuclideanDB(HnswEuclideanDB),
    HnswCosineDB(HnswCosineDB),
}

pub fn new(db_method: &str) -> DB {
    match db_method {
        "Cosine" => DB::CosineDB(CosineDB { docs: Vec::new() }),
        "Euclidean" => DB::EuclideanDB(EuclideanDB { docs: Vec::new() }),
        "Hnsw_Euclidean" => DB::HnswEuclideanDB(HnswEuclideanDB { docs: Vec::new(), hnsw: Hnsw::new(1, 1, 1, 1, DistL2 {}) }),
        "Hnsw_Cosine" => DB::HnswCosineDB(HnswCosineDB { docs: Vec::new(), hnsw: Hnsw::new(1, 1, 1, 1, DistCosine {}) }),
        _ => DB::CosineDB(CosineDB { docs: Vec::new() }),
    }
}

pub trait Operations {
    fn load(&mut self, texts: Vec<String>);
    fn query(&self, query: String, n: u32) -> Vec<Doc>;
}

impl Operations for DB {
    fn load(&mut self, texts: Vec<String>) {
        match self {
            DB::CosineDB(db) => db.load(texts),
            DB::EuclideanDB(db) => db.load(texts),
            DB::HnswEuclideanDB(db) => db.load(texts),
            DB::HnswCosineDB(db) => db.load(texts),
        }
    }

    fn query(&self, query: String, n: u32) -> Vec<Doc> {
        match self {
            DB::CosineDB(db) => db.query(query, n),
            DB::EuclideanDB(db) => db.query(query, n),
            DB::HnswEuclideanDB(db) => db.query(query, n),
            DB::HnswCosineDB(db) => db.query(query, n),
        }
    }
}