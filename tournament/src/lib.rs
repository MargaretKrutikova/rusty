use itertools::Itertools;
use std::cmp::Ord;
use std::collections::HashMap;
use std::fmt::Display;

struct TeamStats {
    matches_won: i32,
    matches_drawn: i32,
    matches_lost: i32,
}

impl TeamStats {
    pub fn new() -> Self {
        TeamStats {
            matches_won: 0,
            matches_lost: 0,
            matches_drawn: 0,
        }
    }
    pub fn calc_team_points(&self) -> i32 {
        self.matches_won * 3 + self.matches_drawn
    }

    pub fn calc_total_matches(&self) -> i32 {
        self.matches_won + self.matches_lost + self.matches_drawn
    }
    pub fn add_win(stats: &TeamStats) -> TeamStats {
        TeamStats {
            matches_won: stats.matches_won + 1,
            ..*stats
        }
    }
    pub fn add_draw(stats: &TeamStats) -> TeamStats {
        TeamStats {
            matches_drawn: stats.matches_drawn + 1,
            ..*stats
        }
    }
    pub fn add_loss(stats: &TeamStats) -> TeamStats {
        TeamStats {
            matches_lost: stats.matches_lost + 1,
            ..*stats
        }
    }
}

struct CompetitionResult {
    table: HashMap<String, TeamStats>,
}

impl<'a> CompetitionResult {
    pub fn new() -> Self {
        CompetitionResult {
            table: HashMap::new(),
        }
    }

    fn update_team(
        self,
        team: &String,
        update_fn: impl Fn(&TeamStats) -> TeamStats,
    ) -> CompetitionResult {
        let mut mutable_self = self;
        let new_result = match mutable_self.table.get(team) {
            Some(team_result) => update_fn(team_result),
            None => update_fn(&TeamStats::new()),
        };
        mutable_self.table.insert(team.clone(), new_result);
        mutable_self
    }
    pub fn add_win(self, team: &String) -> CompetitionResult {
        self.update_team(team, |stats| TeamStats::add_win(stats))
    }
    pub fn add_loss(self, team: &String) -> CompetitionResult {
        self.update_team(team, |stats| TeamStats::add_loss(stats))
    }
    pub fn add_draw(self, team: &String) -> CompetitionResult {
        self.update_team(team, |stats| TeamStats::add_draw(stats))
    }
}

enum MatchResult {
    Win { winner: String, loser: String },
    Draw((String, String)),
}

// ---- Parsing ----

fn create_winner_result(winner: &str, loser: &str) -> MatchResult {
    MatchResult::Win {
        winner: winner.to_string(),
        loser: loser.to_string(),
    }
}

fn create_draw_result(team1: &str, team2: &str) -> MatchResult {
    MatchResult::Draw((team1.to_string(), team2.to_string()))
}

fn parse_match_result(result: &str) -> Option<MatchResult> {
    let split = result.split(";").collect::<Vec<_>>();
    match split.get(2) {
        Some(&"win") => Some(create_winner_result(split[0], split[1])),
        Some(&"loss") => Some(create_winner_result(split[1], split[0])),
        Some(&"draw") => Some(create_draw_result(split[0], split[1])),
        _ => None,
    }
}

fn parse_results<'a>(match_results: &str) -> CompetitionResult {
    match_results.split("\n").fold(
        CompetitionResult::new(),
        |acc, result| match parse_match_result(result) {
            Some(MatchResult::Win { winner, loser }) => acc.add_win(&winner).add_loss(&loser),
            Some(MatchResult::Draw((team1, team2))) => acc.add_draw(&team1).add_draw(&team2),
            _ => acc,
        },
    )
}

// ---- Presentation ----

fn format_table_row<T>(header: &String, cells: &Vec<T>) -> String
where
    T: Display,
{
    let cells_str = cells
        .iter()
        .map(|val| format!("| {: >2}", val))
        .collect::<Vec<String>>()
        .join(" ");
    format!("{: <31}{}", header, cells_str)
}

fn format_table_header() -> String {
    format_table_row(&String::from("Team"), &vec!["MP", "W", "D", "L", "P"])
}

fn format_team_result(team: &String, result: &TeamStats) -> String {
    let cells = vec![
        result.calc_total_matches(),
        result.matches_won,
        result.matches_drawn,
        result.matches_lost,
        result.calc_team_points(),
    ];
    format_table_row(team, &cells)
}

fn sort_stats(left: &(&String, &TeamStats), right: &(&String, &TeamStats)) -> std::cmp::Ordering {
    match (left.1.calc_team_points(), right.1.calc_team_points()) {
        (x, y) if x == y => Ord::cmp(&left.0, &right.0),
        (x, y) => Ord::cmp(&x, &y).reverse(),
    }
}

fn format_competition_result(result: &CompetitionResult) -> String {
    let rows = result
        .table
        .iter()
        .sorted_by(|left, right| sort_stats(left, right))
        .map(|(key, value)| format_team_result(key, value))
        .collect::<Vec<String>>()
        .join("\n");
    match rows.is_empty() {
        true => format_table_header(),
        false => format!("{}\n{}", format_table_header(), rows),
    }
}

pub fn tally(match_results: &str) -> String {
    let competition_result = parse_results(match_results);
    format_competition_result(&competition_result)
}
