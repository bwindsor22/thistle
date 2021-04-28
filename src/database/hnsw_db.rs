use crate::database::embedding::get_embedding;
use crate::database::db::{Operations, Doc};
use crate::hnswlib::*;

pub struct HnswDB {
    pub docs: Vec<Doc>,
    pub hnsw: Hnsw<f64, DistL2>,
}

impl Operations for HnswDB {
    fn load(&mut self, texts: Vec<String>) {
        let nb_elem = texts.len();
        let data: Vec<_> = texts.iter().map(|x| get_embedding(&x)).collect();
        let data_with_id: Vec<_> = data.iter().zip(0..nb_elem).collect();

        let ef_c = 200;
        let max_nb_connection = 15;
        let nb_layer = 16.min((nb_elem as f32).ln().trunc() as usize);
        let hnsw = Hnsw::<f64, DistL2>::new(max_nb_connection, nb_elem, nb_layer, ef_c, DistL2 {});
        hnsw.parallel_insert(&data_with_id);
        self.hnsw = hnsw;

        for text in texts {
            self.docs.push(Doc {
                text: text,
                embedding: Vec::new(),
                score: 0.0,
            });
        }
    }

    fn query(&self, query: String, n: u32) -> Vec<Doc> {
        // let mut result = Vec::new();
        // let query_embedding = get_embedding(&query);
        // for doc in self.docs.iter() {
        //     let score = euclidean(&doc.embedding, &query_embedding);
        //     result.push(Doc {
        //         text: doc.text.clone(),
        //         embedding: doc.embedding.clone(),
        //         score: score,
        //     });
        // }

        // result.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

        // result.drain(..n as usize).collect()
        let query_embedding = get_embedding(&query);
        let max_nb_connection = 15;
        let ef_arg = max_nb_connection * 2;
        let neighbors = self.hnsw.search(&query_embedding, n as usize, ef_arg);
        
        let mut res = Vec::new();
        for neighbor in neighbors {
            res.push(Doc {
                text: self.docs[neighbor.d_id].text.clone(),
                embedding: self.docs[neighbor.d_id].embedding.clone(),
                score: neighbor.distance as f64,
            })
        }
        // let res = neighbors.map(|neighbor| self.docs[neighbor.d_id].clone()).collect();
        res
    }
}
