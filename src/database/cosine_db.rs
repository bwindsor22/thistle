use uuid::Uuid;
use std::slice;
use crate::database::embeddings::get_embedding;

pub struct Doc {
    text: String,
    embedding: Vec<f64>,
    similarity: f64,
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
                similarity: 0.0,
            })
        }
        DB { docs }
    }

    pub fn query(self, query: String, n: u32) -> Vec<Doc> {
        let mut result = Vec::new();
        let query_embedding = get_embedding(&query);
        for doc in self.docs.iter() {
            let similarity = cosine(&doc.embedding, &query_embedding);
            result.push(Doc {
                text: doc.text.clone(),
                embedding: doc.embedding.clone(),
                similarity: similarity,
            });
        }

        result.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

        result.drain(..n as usize).collect()
    }
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
