use uuid::Uuid;
use rand::Rng;

pub struct Doc {
    text: String,
    embedding: Vec<f64>
}

impl Doc {
    pub fn get_text(&self) -> String{
        self.text.clone()
    }
}

pub struct DB {
    docs: Vec<Doc>
}

impl DB {
    pub fn load(texts: Vec<String>) -> Self {
        let mut docs = Vec::new();
        for text in texts {
            let vect = get_embedding(text.clone());
            docs.push(Doc{text:text, embedding:vect})
        }
        DB{docs}
    }

    pub fn query(self, query: String, threshold: f64) -> Vec<Doc>{
        let mut result = Vec::new();
        let query_embedding = get_embedding(query);
        for doc in self.docs.iter(){
            let embed = doc.embedding.clone();
            if cosine(embed, query_embedding.clone()) > threshold  {
                result.push(Doc{text:doc.text.clone(), embedding:doc.embedding.clone()});
            }
        }
        result
    }
}
// todo: convert from random numbers to BERT embedding
fn get_embedding(_text: String) -> Vec<f64>{
    let mut rng = rand::thread_rng();
    let vals: Vec<f64> = (0..100).map(|_| rng.gen_range(0.0, 1.0)).collect();
    vals
}

// todo: normalize to 1
fn cosine(vec1: Vec<f64>, vec2: Vec<f64>) -> f64{
    vec1.iter().zip(vec2.iter())
        .fold(0.0, |sum, (&v1, &v2)| sum + (v1 * v2))
}


pub fn database_module_uuid() -> String {
    Uuid::new_v4().to_string()
}

