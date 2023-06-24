use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::model::SomeStruct;

pub struct SomeController;
impl SomeController {
    pub async fn get_something(
        Path(id): Path<usize>,
        State(db): State<Arc<Mutex<HashMap<String, String>>>>, // <- if you need your database in your controller
    ) -> Result<Json<SomeStruct>, StatusCode> {
        let some_struct = SomeStruct {
            some_field: id.to_string(),
        };

        Ok(Json(some_struct))
    }
}
