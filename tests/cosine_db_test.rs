#![feature(array_map)]
use thistle::database::Operations;

#[test]
fn make_uuid() {
    println!("local module uuid is {}", thistle::filemod::get_uuid());

    assert_eq!(0, 0);
}

#[test]
fn run_cosine_db() {
    let texts = [
        "Do not go gentle into that good night",
        "Shall I compare thee to a summer's day",
        "What happens to a dream deferred?"
    ]
    .map(|x| x.to_string())
    .to_vec();
    let mut db = thistle::database::new("Cosine");
    db.load(texts);
    let result = db.query("stay strong as you grow older".to_string(), 1);
    // println!("{:?}", result);
    assert_eq!("Do not go gentle into that good night", result[0].text);
}