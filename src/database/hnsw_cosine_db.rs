use crate::database::embedding::get_embedding;
use crate::database::db::{Operations, Doc};
use crate::hnswlib::*;

pub struct HnswCosineDB {
    pub docs: Vec<Doc>,
    pub hnsw: Hnsw<f64, DistCosine>,
}

impl Operations for HnswCosineDB {
    fn load(&mut self, texts: Vec<String>) {
        let nb_elem = texts.len();
        let mut data = Vec::new();
        for text in texts {
            let vect = get_embedding(&text);
            self.docs.push(Doc {
                text: text,
                embedding: vect.clone(),
                score: 0.0,
            });
            data.push(vect);
        }
        let data_with_id: Vec<_> = data.iter().zip(0..nb_elem).collect();
        let ef_c = 200;
        let max_nb_connection = 15;
        let nb_layer = 16.min((nb_elem as f32).ln().trunc() as usize);
        let hnsw = Hnsw::<f64, DistCosine>::new(max_nb_connection, nb_elem, nb_layer, ef_c, DistCosine {});
        hnsw.parallel_insert(&data_with_id);
        self.hnsw = hnsw;
    }

    fn query(&self, query: String, n: u32) -> Vec<Doc> {
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
        res
    }
}
