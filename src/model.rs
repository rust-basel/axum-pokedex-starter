#[derive(serde::Serialize, serde::Deserialize)] // <- makes your struct serializable
pub struct SomeStruct {
    pub some_field: String,
}
