use crate::database::embedding::get_embedding;
use crate::database::db::{Operations, Doc};

pub struct EuclideanDB {
    pub docs: Vec<Doc>,
}

impl Operations for EuclideanDB {
    fn load(&mut self, texts: Vec<String>) {
        for text in texts {
            let vect = get_embedding(&text);
            self.docs.push(Doc {
                text: text,
                embedding: vect,
                score: 0.0,
            });
        }
    }

    fn query(&self, query: String, n: u32) -> Vec<Doc> {
        let mut result = Vec::new();
        let query_embedding = get_embedding(&query);
        for doc in self.docs.iter() {
            let score = euclidean(&doc.embedding, &query_embedding);
            result.push(Doc {
                text: doc.text.clone(),
                embedding: doc.embedding.clone(),
                score: score,
            });
        }

        result.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

        result.drain(..n as usize).collect()
    }
}

fn euclidean(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    let res = v1.iter()
                .zip(v2.iter())
                .map(|(x, y)| (x - y).powi(2))
                .sum::<f64>()
                .sqrt();
    // For dev purposes
    println!("Euclidean {}", res);
    res
}