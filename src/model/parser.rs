use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RankingPosition {
    player: String,
    kills: i16,
    position: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MatchRecord {
    total_kills: i16,
    players: HashSet<String>,
    kills: HashMap<String, i16>,
    kills_by_means: HashMap<String, i16>,
    ranking: Vec<RankingPosition>,
}

pub struct Parser {}

impl Parser {
    pub fn parse(file_path: String) -> HashMap<String, MatchRecord> {
        let file_buffer = Parser::load_file(file_path.to_string());

        let mut matches: HashMap<String, MatchRecord> = HashMap::new();
        let mut current_match = String::new();

        for raw_line in file_buffer.lines() {
            let line = match raw_line {
                Ok(line) => line.trim().to_string(),
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            };

            Parser::parse_line(line, &mut matches, &mut current_match)
        }

        dbg!(matches.clone());
        matches
    }

    fn load_file(file_path: String) -> BufReader<File> {
        let reader = BufReader::new(File::open(file_path).expect("Cannot open {file_path}"));

        reader
    }

    fn parse_line(
        line: String,
        matches: &mut HashMap<String, MatchRecord>,
        current_match: &mut String,
    ) {
        let new_match_started = line.contains("InitGame: ");
        let user_joined = line.contains("ClientUserinfoChanged: ");
        let user_got_killed = line.contains("Kill: ");

        let match_has_ended = line.contains("ShutdownGame: ");
        let match_break =
            line.contains("------------------------------------------------------------");

        let ranking_is_empty = match matches.get(current_match) {
            Some(match_record) => match_record.ranking.is_empty(),
            None => false,
        };

        if new_match_started {
            Parser::handle_match_start(matches, current_match);
        } else if user_joined {
            Parser::handle_user_joined_match(line, matches, &current_match);
        } else if user_got_killed {
            Parser::handle_user_kills(line, matches, &current_match);

        // This validation is necessary because a match doesn't have a ShutdownGame event, although it has all the others events
        } else if match_has_ended || (match_break && !current_match.is_empty() && ranking_is_empty)
        {
            Parser::generate_ranking(matches, &current_match)
        }
    }

    fn handle_match_start(matches: &mut HashMap<String, MatchRecord>, current_match: &mut String) {
        let current_match_number = matches.keys().len();
        *current_match = format!("game_{current_match_number}").to_string();

        matches.insert(
            current_match.clone(),
            MatchRecord {
                total_kills: 0,
                players: HashSet::new(),
                kills: HashMap::new(),
                kills_by_means: HashMap::new(),
                ranking: Vec::new(),
            },
        );
    }

    fn handle_user_joined_match(
        line: String,
        matches: &mut HashMap<String, MatchRecord>,
        current_match: &String,
    ) {
        let mut split_line = line.split("\\");
        let username = split_line.nth(1).unwrap();

        if let Some(match_record) = matches.get_mut(current_match) {
            match_record.players.insert(username.to_string());
            match_record.kills.insert(username.to_string(), 0);
        }
    }

    fn handle_user_kills(
        line: String,
        matches: &mut HashMap<String, MatchRecord>,
        current_match: &String,
    ) {
        let game_environment = "<world>";

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

        if let Some(match_record) = matches.get_mut(current_match) {
            match_record.total_kills += 1;

            if killer == game_environment {
                Parser::remove_kill_score_from_user(match_record, killed);
            } else if killer != killed {
                Parser::add_kill_score_to_user(match_record, killer);
            }

            Parser::register_kill_means(match_record, weapon);
        }
    }

    fn add_kill_score_to_user(match_record: &mut MatchRecord, killer: &str) {
        match_record
            .kills
            .entry(killer.to_string())
            .and_modify(|k| *k += 1)
            .or_insert(0);
    }

    fn remove_kill_score_from_user(match_record: &mut MatchRecord, killed: &str) {
        match_record
            .kills
            .entry(killed.to_string())
            .and_modify(|k| *k -= 1)
            .or_insert(0);
    }

    fn register_kill_means(match_record: &mut MatchRecord, weapon: &str) {
        match_record
            .kills_by_means
            .entry(weapon.to_string())
            .and_modify(|k| *k += 1)
            .or_insert(0);
    }

    fn generate_ranking(matches: &mut HashMap<String, MatchRecord>, current_match: &String) {
        if let Some(match_record) = matches.get_mut(current_match) {
            let mut raw_ranking = match_record.kills.iter().collect::<Vec<(&String, &i16)>>();
            raw_ranking.sort_by(|a, b| b.1.cmp(a.1));

            let ranking = raw_ranking
                .iter()
                .enumerate()
                .map(|(i, (player, kills))| RankingPosition {
                    player: player.to_string(),
                    kills: **kills,
                    position: (i + 1) as u16,
                })
                .collect::<Vec<RankingPosition>>();

            match_record.ranking = ranking;
        }
    }
}
