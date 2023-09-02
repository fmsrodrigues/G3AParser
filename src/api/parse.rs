use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
use axum::{extract::Multipart,  http::{StatusCode}, Json};
use std::io::Write;
use std::{fs, path};

use std::collections::HashMap;

use crate::model::parser::{Parser, MatchRecord};

#[axum_macros::debug_handler]
pub async fn parse_log_file(mut multipart: Multipart) -> Result<Json<HashMap<String, MatchRecord>>, (StatusCode, String)> {
    let mut parsed_log = HashMap::new();

    while let Some(field) = multipart.next_field().await.map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))? {
        let is_text_file = field.content_type().unwrap().to_string() == "text/plain";
        let is_log_field = field.name().unwrap().to_string() == "log";

        if is_text_file && is_log_field {
            let base_path = "./tmp/";
            let random_prefix_filename = uuid::Uuid::new_v4().to_string();
            let complete_path = format!("{base_path}{random_prefix_filename}_log.txt");
            let file_path = path::Path::new(complete_path.as_str());

            let data = field.bytes().await.unwrap();

            let mut file = fs::File::create(&file_path).map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
            file.write_all(&data).map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

            parsed_log = Parser::parse(String::from(file_path.to_str().unwrap()));

            fs::remove_file(file_path).map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        } else {
            println!("Should return an error here");

            return Err((StatusCode::BAD_REQUEST, "Invalid file type or field name".to_string()));
        }
    }

    Ok(Json(parsed_log))
}
}}
