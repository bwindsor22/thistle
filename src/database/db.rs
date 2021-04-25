use crate::database::cosine_db::CosineDB;

#[derive(Debug)]
pub struct Doc {
    pub text: String,
    pub embedding: Vec<f64>,
    pub similarity: f64,
}

#[derive(Debug)]
pub enum DB {
    Cosine(CosineDB),
    // Euclidean(EuclideanDB),
}

pub trait Operations {
    fn load(&mut self, texts: Vec<String>);
    fn query(&self, query: String, n: u32) -> Vec<Doc>;
}

pub fn new(db_method: &str) -> DB {
    match db_method {
        "Cosine" => DB::Cosine(CosineDB { docs: Vec::new() }),
        _ => DB::Cosine(CosineDB { docs: Vec::new() }),

    }
}

impl Operations for DB {
    fn load(&mut self, texts: Vec<String>) {
        match self {
            DB::Cosine(db) => db.load(texts),
            // DB::CosineDB(db) => db.load(),
        }
    }

    fn query(&self, query: String, n: u32) -> Vec<Doc> {
        match self {
            DB::Cosine(db) => db.query(query, n),
            // DB::CosineDB(db) => db.load(),
        }
    }
}