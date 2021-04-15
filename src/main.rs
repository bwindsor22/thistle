use uuid::Uuid;

mod filemod;
use filemod::get_uuid;

mod foldermodule;
use foldermodule::foldermodulefile::folder_module_uuid;

mod database;
use database::database_fns::{database_module_uuid, DB};

fn main() {
    // Example code to show how modules work, will remove
    let file_mod_uuid = get_uuid();
    println!("local module uuid is {}", file_mod_uuid);
    let folder_mod_uuid = folder_module_uuid();
    println!("folder module uuid is {}", folder_mod_uuid);
    let database_module_uuid = database_module_uuid();
    println!("database_module_uuid is {}", database_module_uuid);

    let texts = [
        "do not go gentle".to_string(),
        "into that good night".to_string(),
        "old age should burn and rave".to_string(),
    ]
    .to_vec();
    let db = DB::load(texts);
    let result = db.query("at close of day".to_string(), 0.5);
    for doc in result {
        println!("{}", doc.get_text());
    }
}
