#![feature(array_map)]
use thistle::database::Operations;

#[test]
fn run_euclidean_db() {
    let texts = [
        "Do not go gentle into that good night",
        "Shall I compare thee to a summer's day",
        "What happens to a dream deferred?"
    ]
    .map(|x| x.to_string())
    .to_vec();
    let mut db = thistle::database::new("Euclidean");
    db.load(texts);
    let result = db.query("stay strong as you grow older".to_string(), 1);
    // println!("{:?}", result);
    assert_eq!("Do not go gentle into that good night", result[0].text);
}