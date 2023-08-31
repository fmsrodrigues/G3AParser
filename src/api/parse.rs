use leptos::*;

#[server(ParseLog, "/api", "Url", "parse_log")]
pub async fn parse_log(title: String) -> Result<String, ServerFnError> {
    use std::collections::{HashMap, HashSet};
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    struct MatchRecord {
        total_kills: i16,
        players: HashSet<String>,
        kills: HashMap<String, i16>,
        kills_by_means: HashMap<String, i16>,
    }

    let file_path = "./src/api/qgames.log";

    let reader = BufReader::new(File::open(file_path).expect("Cannot open {file_path}"));

    let mut matches: HashMap<String, MatchRecord> = HashMap::new();
    let mut current_match = String::new();
    for raw_line in reader.lines() {
        let line = match raw_line {
            Ok(line) => line.trim().to_string(),
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        let new_match_started = line.contains("InitGame:");
        let user_joined = line.contains("ClientUserinfoChanged:");
        let user_got_killed = line.contains("Kill:");

        let game_environment = "<world>";

        if new_match_started {
            let current_match_number = matches.keys().len();
            current_match = format!("game_{current_match_number}").to_string();

            matches.insert(
                current_match.clone(),
                MatchRecord {
                    total_kills: 0,
                    players: HashSet::new(),
                    kills: HashMap::new(),
                    kills_by_means: HashMap::new(),
                },
            );

            continue;
        }

        if user_joined {
            let mut split_line = line.split("\\");
            let username = split_line.nth(1).unwrap();

            if let Some(match_record) = matches.get_mut(&current_match) {
                match_record.players.insert(username.to_string());
                match_record.kills.insert(username.to_string(), 0);
            }

            continue;
        }

        if user_got_killed {
            let split_line = line.split(": ").nth(2).unwrap();
            let killer = split_line.split("killed").nth(0).unwrap().trim();
            let killed = split_line
                .split("killed")
                .nth(1)
                .unwrap()
                .split("by")
                .nth(0)
                .unwrap()
                .trim();
            let weapon = split_line.split("by").nth(1).unwrap().trim();

            if let Some(match_record) = matches.get_mut(&current_match) {
                match_record.total_kills += 1;

                if (killer == game_environment) {
                    match_record
                        .kills
                        .entry(killed.to_string())
                        .and_modify(|k| *k -= 1)
                        .or_insert(0);
                } else if killer != killed {
                    match_record
                        .kills
                        .entry(killer.to_string())
                        .and_modify(|k| *k += 1)
                        .or_insert(0);
                }

                match_record
                    .kills_by_means
                    .entry(weapon.to_string())
                    .and_modify(|k| *k += 1)
                    .or_insert(0);
            }
            continue;
        }
    }

    dbg!(matches);
    Ok(title)
}
