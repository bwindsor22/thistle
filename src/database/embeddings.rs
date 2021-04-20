use std::path::Path;
use tch::{Tensor, Kind, Device, nn, Cuda, no_grad};
use crate::model::SentenceTransformer;


pub fn get_embedding(text: &str) -> Vec<f64> {
    let device = Device::Cpu;

    let embedder = SentenceTransformer::new(
        Path::new("models/bert-base-nli-stsb-mean-tokens"),
        device).unwrap();

    let embedding = embedder.encode(text);
    // For dev purposes
    println!("&embedding[..5] of {}: {:?}", text, &embedding[..5]);
    embedding
}
