use uuid::Uuid;

pub fn folder_module_uuid() -> String {
    Uuid::new_v4().to_string()
}
