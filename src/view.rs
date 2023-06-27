#[derive(serde::Serialize, serde::Deserialize)] // <- makes your struct serializable
pub struct SomeView {
    pub some_field: String,
}
