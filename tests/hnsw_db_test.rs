use std::time::{Duration, SystemTime};
use cpu_time::ProcessTime;
use rand::distributions::{Uniform};
use rand::prelude::*;

use thistle::hnswlib::*;

// The current test performs the following:
// * generate `nb_elem` elements, each of dim `dim`
// * Hnsw is created to insert
// * a random vector is generated and queried
// * `knbn` neighbors are returned
#[test]
fn run_hnsw_db() {
    let nb_elem = 50;
    let dim = 25;
    let data = gen_random_matrix_f64(dim, nb_elem);
    println!("{:?}", data);
    let data_with_id = data.iter().zip(0..data.len()).collect();
    // println!("{:?}", data_with_id);

    let ef_c = 200;
    let max_nb_connection = 15;
    let nb_layer = 16.min((nb_elem as f32).ln().trunc() as usize);
    let hnsw = Hnsw::<f64, DistL2>::new(max_nb_connection, nb_elem, nb_layer, ef_c, DistL2{});
    let start = ProcessTime::now();
    let begin_t = SystemTime::now();
    hnsw.parallel_insert(&data_with_id);
    let cpu_time: Duration = start.elapsed();
    println!(" hnsw data insertion  cpu time {:?}", cpu_time); 
    println!(" hnsw data insertion parallel,   system time {:?} \n", begin_t.elapsed().unwrap()); 
    hnsw.dump_layer_info();
    println!(" parallel hnsw data nb point inserted {:?}", hnsw.get_nb_point());

    let ef_search = max_nb_connection * 2;
    let knbn = 3;
    let mut r_vec = Vec::<f64>::with_capacity(dim);
    let mut rng = thread_rng();
    let unif = Uniform::<f64>::new(0.,1.);
    for _ in 0..dim {
        r_vec.push(rng.sample(unif));
    }
    let neighbors = hnsw.search(&r_vec, knbn, ef_search);
    println!("{:?}", neighbors);
}