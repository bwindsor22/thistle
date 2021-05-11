use crate::lsh::prelude::LshMem;
use crate::database::embedding::{get_embedding, EMBEDDING_SIZE};
use crate::database::db::{Operations, Doc};

pub struct LshDB {
    pub docs: Vec<Doc>,
    pub lsh: crate::lsh::lsh::LSH<crate::lsh::hash::SignRandomProjections<f64>,
                                    f64,
                                    crate::lsh::table::mem::MemoryTable<f64, i8>>,
}

impl Operations for LshDB {
    fn load(&mut self, texts: Vec<String>) {
        let mut vecs = Vec::new();
        for text in texts {
            let vect = get_embedding(&text);
            self.docs.push(Doc {
                text: text,
                embedding: vect.clone(),
                score: 0.0,
            }); 
            vecs.push(vect);
        }
        let n_projections = 9;
        let n_hash_tables = 30;
        let dim = EMBEDDING_SIZE;
        let mut lsh = LshMem::new(n_projections, n_hash_tables, dim as usize)
        .srp()
        .unwrap();
        lsh.store_vecs(&vecs);
        self.lsh = lsh;
        println!("loaded");
    }

    fn query(&self, query: String, n: u32) -> Vec<Doc> {
        let query_embedding = get_embedding(&query);
        let mut matching = self.lsh.query_bucket_ids(&query_embedding).unwrap();
        let mut output = Vec::new();
        if matching.len() == 0 {
            return output;
        }
        // println!("matching {:?}", matching);
        let top_n: Vec<u32> = matching.drain(..n as usize).collect();
        for match_ in top_n {
            let doc = &self.docs[match_ as usize];
            output.push(Doc{
                text: doc.text.clone(),
                embedding: doc.embedding.clone(),
                score: 1.0,
            });
        }
        output
    }
}

