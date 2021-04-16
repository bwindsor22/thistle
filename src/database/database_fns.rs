use rand::Rng;
use uuid::Uuid;
// use std::env;
// use std::path::PathBuf;
// use std::time::Instant;

// use rand::{rngs::StdRng, Rng, SeedableRng};
// use rust_tokenizers::tokenizer::{BertTokenizer, Tokenizer, TruncationStrategy};
// use tokenizers::models::wordpiece::WordPiece;
// use tokenizers::normalizers::bert::BertNormalizer;
// use tokenizers::pre_tokenizers::bert::BertPreTokenizer;
// use tokenizers::processors::bert::BertProcessing;
// use tokenizers::tokenizer;
// use torch_sys::dummy_cuda_dependency;

// use sbert::Tokenizer as TraitTokenizer;
// use sbert::{HFTokenizer, SBertHF, SBertRT};

// const BATCH_SIZE: usize = 64;

// fn rand_string(r: &mut impl Rng) -> String {
//     (0..(r.gen::<f32>() * 100.0) as usize)
//         .map(|_| (0x20u8 + (r.gen::<f32>() * 96.0) as u8) as char)
//         .collect()
// }

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
            let vect = get_embedding(text.clone());
            docs.push(Doc {
                text: text,
                embedding: vect,
            })
        }
        DB { docs }
    }

    pub fn query(self, query: String, threshold: f64) -> Vec<Doc> {
        let mut result = Vec::new();
        let query_embedding = get_embedding(query);
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
// todo: convert from random numbers to BERT embedding
fn get_embedding(_text: String) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let vals: Vec<f64> = (0..100).map(|_| rng.gen_range(0.0, 1.0)).collect();
    vals
    // unsafe {
    //     dummy_cuda_dependency();
    // } // Windows Hack
    // let mut r = StdRng::seed_from_u64(42);

    // let mut home: PathBuf = env::current_dir().unwrap();
    // home.push("models");
    // home.push("distiluse-base-multilingual-cased");

    // println!("Loading sbert ...");
    // let before = Instant::now();
    // let sbert_model = SBertRT::new(home).unwrap();
    // println!("Elapsed time: {:.2?}", before.elapsed());

    // let mut texts = Vec::new();
    // texts.push(String::from("TTThis player needs tp be reported lolz."));
    // for _ in 0..9 {
    //     texts.push(rand_string(&mut r));
    // }

    // println!("Encoding {} sentences...", texts.len());
    // let before = Instant::now();
    // for _ in 0..9 {
    //     &sbert_model.forward(&texts, BATCH_SIZE).unwrap();
    // }
    // let output = &sbert_model.forward(&texts, BATCH_SIZE).unwrap()[0][..5];
    // println!("Elapsed time: {:?}ms", before.elapsed().as_millis() / 10);
    // println!("Vec: {:?}", output);

    // let v = output
    //     .iter()
    //     .map(|f| (f * 10000.0).round() / 10000.0)
    //     .collect::<Vec<_>>();
    // // assert_eq!(v, [-0.0227, -0.006, 0.0552, 0.0185, -0.0754]);
    // v
}

fn cosine(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    let norms = norm(vec1) * norm(vec2);
    if norms > 0. {
        return dot(vec1, vec2) / (norm(vec1) * norm(vec2));
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
