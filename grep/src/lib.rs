use anyhow::Error;
use std::fs;

#[derive(Debug, PartialEq)]
enum Flag {
    PrintLineNumber,
    PrintFileNameOnly,
    CaseInsensitive,
    InvertResults,
    MatchEntireLine,
}

#[derive(Debug)]
pub struct Flags {
    list: Vec<Flag>,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let list = flags
            .iter()
            .filter_map(|flag| Flags::parse_flag(flag))
            .collect();
        Flags { list }
    }
    fn parse_flag(flag: &str) -> Option<Flag> {
        match flag {
            "-n" => Some(Flag::PrintLineNumber),
            "-l" => Some(Flag::PrintFileNameOnly),
            "-i" => Some(Flag::CaseInsensitive),
            "-v" => Some(Flag::InvertResults),
            "-x" => Some(Flag::MatchEntireLine),
            _ => None,
        }
    }
}

fn match_line(pattern: &str, flags: &Vec<Flag>, line: &str) -> bool {
    match flags.contains(&Flag::MatchEntireLine) {
        true if line == pattern => true,
        false if line.contains(pattern) => true,
        _ => false,
    }
}

fn has_match(pattern: &str, flags: &Vec<Flag>, line: &str) -> bool {
    let matches_pattern = match flags.contains(&Flag::CaseInsensitive) {
        true => match_line(&pattern.to_lowercase(), flags, &line.to_lowercase()),
        false => match_line(pattern, flags, line),
    };

    match flags.contains(&Flag::InvertResults) {
        false => matches_pattern,
        true if !line.is_empty() => !matches_pattern,
        _ => false,
    }
}

fn format_match(
    include_line_number: bool,
    include_file_name: bool,
    file_name: &str,
    (index, line): (usize, &str),
) -> String {
    let formatted_line = match include_line_number {
        true => format!("{}:{}", index + 1, line),
        false => String::from(line),
    };
    match include_file_name {
        true => format!("{}:{}", file_name, formatted_line),
        false => formatted_line,
    }
}

fn match_file_content(
    pattern: &str,
    flags: &Vec<Flag>,
    include_file_name: bool,
    file_name: &str,
    content: &str,
) -> Vec<String> {
    let mut matches = content
        .split("\n")
        .enumerate()
        .filter(|(_, line)| has_match(pattern, flags, line));

    match flags.contains(&Flag::PrintFileNameOnly) {
        true => match matches.next() {
            Some(_) => vec![String::from(file_name)],
            None => vec![],
        },
        false => {
            let include_line_number = flags.contains(&Flag::PrintLineNumber);
            matches
                .map(|pair| format_match(include_line_number, include_file_name, file_name, pair))
                .filter(|result| !result.is_empty())
                .collect::<Vec<_>>()
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut matching_lines: Vec<String> = Vec::new();
    let include_file_name = files.len() > 1;

    for file_name in files.iter() {
        let content = fs::read_to_string(file_name)?;
        let matches =
            match_file_content(pattern, &flags.list, include_file_name, file_name, &content);

        matching_lines.extend(matches);
    }
    Ok(matching_lines)
}
