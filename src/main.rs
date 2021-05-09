use thistle::prelude::LshMem;

fn main() {
    println!("run");
    // 2 rows w/ dimension 3.
    let p = &[vec![1., 1.5, 2.],
    vec![2., 1.1, -0.3]];

    // Do one time expensive preprocessing.
    let n_projections = 9;
    let n_hash_tables = 30;
    let dim = 10;
    let dim = 3;
    let mut lsh = LshMem::new(n_projections, n_hash_tables, dim)
    .srp()
    .unwrap();
    lsh.store_vecs(p);

    // Query in sublinear time.
    let query = &[1.1, 1.2, 1.2];
    let out = lsh.query_bucket(query).unwrap();

    for vecs in out {
        println!("{:?}", vecs);
    }
}
