use leptos::*;

#[server(ParseLog, "/api", "Url", "parse_log")]
pub async fn parse_log(title: String) -> Result<String, ServerFnError> {
    use std::env;
    use std::fs;

    let file_path = "./src/api/qgames.log";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    log!("{}", title);
    Ok(title)
}
