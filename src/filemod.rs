use uuid::Uuid;

pub fn get_uud() -> String {
    Uuid::new_v4().to_string()
}
