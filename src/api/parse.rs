use cfg_if::cfg_if;
use std::collections::HashMap;

use leptos::*;

use crate::model::parser::MatchRecord;

// TODO: Remove this when we have a real parser
#[server(ParseLog, "/api", "Url", "parse_log")]
pub async fn parse_log(title: String) -> Result<HashMap<String, MatchRecord>, ServerFnError> {
    use crate::model::parser::Parser;

    let file_path = "./tmp/qgames.log".to_string();

    let parsed_log = Parser::parse(file_path);

    Ok(parsed_log)
}

cfg_if! { if #[cfg(feature = "ssr")] {
use axum::{extract::Multipart};
use std::io::Write;
use std::{fs, path};
use crate::model::parser::Parser;

pub async fn parse_log_file(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let is_text_file = field.content_type().unwrap().to_string() == "text/plain";
        let is_log_field = field.name().unwrap().to_string() == "log";

        if is_text_file && is_log_field {
            let base_path = "./tmp/";
            let random_prefix_filename = uuid::Uuid::new_v4().to_string();
            let complete_path = format!("{base_path}{random_prefix_filename}_log.txt");
            let file_path = path::Path::new(complete_path.as_str());

            let data = field.bytes().await.unwrap();

            let mut file = fs::File::create(&file_path).expect("failed to create file.");
            file.write_all(&data).expect("failed to write to file.");

            let parsed_log = Parser::parse(String::from(file_path.to_str().unwrap()));

            fs::remove_file(file_path).expect("failed to remove file.");

            dbg!(parsed_log);
        } else {
            println!("Should return an error here");

            return ();
        }
    }
}
}}
