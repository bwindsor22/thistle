#![feature(array_map)]
use thistle::database::Operations;

fn main() {
    let texts = [
        "Do not go gentle into that good night",
        "Shall I compare thee to a summer's day",
        "What happens to a dream deferred?"
    ]
    .map(|x| x.to_string())
    .to_vec();
    let mut db = thistle::database::new("LSH");
    db.load(texts);
    let result = db.query("Don't go into the night".to_string(), 1);
    println!("{:?}", result[0].text);
    println!("len results {:?}", result.len());
}
