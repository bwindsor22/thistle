use crate::database::embedding::get_embedding;
use crate::database::db::{Operations, Doc};

pub struct CosineDB {
    pub docs: Vec<Doc>,
}

impl Operations for CosineDB {
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
            let score = cosine(&doc.embedding, &query_embedding);
            result.push(Doc {
                text: doc.text.clone(),
                embedding: doc.embedding.clone(),
                score: score,
            });
        }

        result.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        result.drain(..n as usize).collect()
    }
}

fn cosine(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    let norms = norm(vec1) * norm(vec2);
    if norms > 0. {
        let res = dot(vec1, vec2) / (norm(vec1) * norm(vec2));
        // For dev purposes
        println!("Cosine {}", res);
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
