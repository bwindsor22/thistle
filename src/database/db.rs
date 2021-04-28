use uuid::Uuid;
use crate::database::cosine_db::CosineDB;
use crate::database::euclidean_db::EuclideanDB;
use crate::database::hnsw_db::HnswDB;
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
    HnswDB(HnswDB),
}

pub fn new(db_method: &str) -> DB {
    match db_method {
        "Cosine" => DB::CosineDB(CosineDB { docs: Vec::new() }),
        "Euclidean" => DB::EuclideanDB(EuclideanDB { docs: Vec::new() }),
        "Hnsw" => DB::HnswDB(HnswDB { docs: Vec::new(), hnsw: Hnsw::new(1, 1, 1, 1, DistL2 {})}),
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
            DB::HnswDB(db) => db.load(texts),
        }
    }

    fn query(&self, query: String, n: u32) -> Vec<Doc> {
        match self {
            DB::CosineDB(db) => db.query(query, n),
            DB::EuclideanDB(db) => db.query(query, n),
            DB::HnswDB(db) => db.query(query, n),
        }
    }
}