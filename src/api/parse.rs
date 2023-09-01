use std::collections::HashMap;

use leptos::*;

use crate::model::parser::MatchRecord;

#[server(ParseLog, "/api", "Url", "parse_log")]
pub async fn parse_log(title: String) -> Result<HashMap<String, MatchRecord>, ServerFnError> {
    use crate::model::parser::Parser;

    let file_path = "./tmp/qgames.log".to_string();

    let parsed_log = Parser::parse(file_path);

    Ok(parsed_log)
}
