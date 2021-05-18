use polars::prelude::*;
use std::fs::File;
use std::time::Instant;
use crate::database::Operations;

pub fn time_and_accuracy() -> () {
    let path = "/Users/bradwindsor/ms_projects/thistle/thistle/data/data_cleaned.tsv";
    let file = File::open(path).expect("could not open file");

    let data = CsvReader::new(file)
            .infer_schema(None)
            .with_delimiter(b"\t"[0])
            .has_header(false)
            .finish()
            .unwrap();
    let row_count = data.shape().0;

    let db_methods = vec!["Cosine", "Euclidean", "Hnsw_Euclidean", "Hnsw_Cosine", "LSH"];
    for method in db_methods.iter() {
        let texts = get_texts(&data, "column_2".to_string());
        let references = texts.clone();
    
        let mut db = crate::database::new(method);
    
        let start_time = Instant::now();
        db.load(texts);
    
        let queries = get_texts(&data, "column_1".to_string());

        // query each item, count # of correct
        let mut correct = 0;
        for i in 0..row_count {
            let query = queries[i].as_str().to_string();
            let query_array = &db.query(query, 1);
            if query_array.len() > 0 {
                let query_result = query_array[0].text.as_str().to_string();
                if &query_result == &references[i] {
                    // println!("correct: expected {:?}, result {:?}", references[i], query_result);
                    correct += 1;
                } else {
                    // println!("expected {:?}, result {:?}", references[i], query_result);
                }
            }
        }
        let elapsed = start_time.elapsed();
        println!("RESULTS: |{}| had correct |{}|, out of |{}|, in |{:?}|", method, correct, row_count, elapsed)
    
    }
}

fn get_texts(data: &DataFrame, column: String) -> Vec<String> {
    let row_count = data.shape().0;
    let text_col = data.select(column.as_str()).unwrap();
    let mut texts = Vec::new();
    for i in 0..row_count {
        let text_cell = &text_col.get(i).unwrap();
        let text = text_cell[0].to_string().clone();
        texts.push(text);
    }
    // println!("{:?}", texts);
    texts
}