use crate::model::SomeModel;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::view::SomeView;

pub struct SomeController;
impl SomeController {
    pub async fn get_something(
        Path(id): Path<usize>,
        State(db): State<Arc<Mutex<HashMap<String, SomeModel>>>>, // <- if you need your database in your controller
    ) -> Result<Json<SomeView>, StatusCode> {
        let db = db.lock().unwrap(); // usually you should handle errors ;)

        let db_key = id.to_string();
        if let Some(model) = db.get(&db_key) {
            let view = SomeView {
                some_field: model.field.clone(),
            };
            Ok(Json(view))
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    }
}
