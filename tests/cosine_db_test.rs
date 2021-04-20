#![feature(array_map)]
extern crate thistle;


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
    let db = thistle::database::cosine_db::DB::load(texts);
    let result = db.query("stay strong as you grow older".to_string(), 1);
    assert_eq!("Do not go gentle into that good night", result[0].get_text());
}