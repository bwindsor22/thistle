// use rand::Rng;
use uuid::Uuid;
use std::path::Path;
use tch::{Tensor, Kind, Device, nn, Cuda, no_grad};
use thistle::model::SentenceTransformer;

pub struct Doc {
    text: String,
    embedding: Vec<f64>,
}

impl Doc {
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

pub struct DB {
    docs: Vec<Doc>,
}

impl DB {
    pub fn load(texts: Vec<String>) -> Self {
        let mut docs = Vec::new();
        for text in texts {
            let vect = get_embedding(&text);
            docs.push(Doc {
                text: text,
                embedding: vect,
            })
        }
        DB { docs }
    }

    pub fn query(self, query: String, threshold: f64) -> Vec<Doc> {
        let mut result = Vec::new();
        let query_embedding = get_embedding(&query);
        for doc in self.docs.iter() {
            if cosine(&doc.embedding, &query_embedding) > threshold {
                result.push(Doc {
                    text: doc.text.clone(),
                    embedding: doc.embedding.clone(),
                });
            }
        }
        result
    }
}

fn get_embedding(text: &str) -> Vec<f64> {
    // let mut rng = rand::thread_rng();
    // let vals: Vec<f64> = (0..100).map(|_| rng.gen_range(0.0, 1.0)).collect();
    // vals

    let device = Device::Cpu;

    let embedder = SentenceTransformer::new(
        Path::new("models/bert-base-nli-stsb-mean-tokens"),
        device).unwrap();

    let embedding = embedder.encode(text);
    // For dev purposes
    println!("&embedding[..5] of {}: {:?}", text, &embedding[..5]);
    embedding
}

fn cosine(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    let norms = norm(vec1) * norm(vec2);
    if norms > 0. {
        let res = dot(vec1, vec2) / (norm(vec1) * norm(vec2));
        // For dev purposes
        println!("cosine {}", res);
        return res;
    }
    return 0.;
}

fn dot(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    vec1.iter()
        .zip(vec2.iter())
        .fold(0.0, |sum, (&v1, &v2)| sum + (v1 * v2))
}

fn norm(a: &Vec<f64>) -> f64 {
    dot(a, a).sqrt()
}

pub fn database_module_uuid() -> String {
    Uuid::new_v4().to_string()
}
