use anyhow::Error;
use std::fs;

#[derive(Debug, PartialEq)]
enum Flag {
    PrintLineNumber,
    PrintFileNameOnly,
    CaseInsensitive,
    FailResultOnly,
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
            "-v" => Some(Flag::FailResultOnly),
            "-x" => Some(Flag::MatchEntireLine),
            _ => None,
        }
    }
}

struct LineMatch<'a> {
    file_name: &'a str,
    line_number: usize,
    line: &'a str,
}

fn format_match(flags: &Vec<Flag>, include_file_name: bool, line_match: &LineMatch) -> String {
    let formatted_line = match flags.contains(&Flag::PrintLineNumber) {
        true => format!("{}:{}", line_match.line_number, line_match.line),
        false => String::from(line_match.line),
    };

    if include_file_name {
        format!("{}:{}", line_match.file_name, formatted_line)
    } else {
        formatted_line
    }
}

fn match_line<'a>(flags: &Vec<Flag>, (pattern, line): (&'a str, &'a str)) -> bool {
    match flags.contains(&Flag::MatchEntireLine) {
        true if line == pattern => true,
        false if line.contains(pattern) => true,
        _ => false,
    }
}

fn has_match(pattern: &str, flags: &Vec<Flag>, line: &str) -> bool {
    let (pattern_transformed, line_transformed) = match flags.contains(&Flag::CaseInsensitive) {
        true => (pattern.to_lowercase(), line.to_lowercase()),
        false => (String::from(pattern), String::from(line)),
    };
    let matches_pattern = match_line(flags, (&pattern_transformed, &line_transformed));
    match flags.contains(&Flag::FailResultOnly) {
        false => matches_pattern,
        true if !line.is_empty() => !matches_pattern,
        _ => false,
    }
}

fn match_file_content<'a>(
    pattern: &'a str,
    flags: &'a Vec<Flag>,
    include_file_name: bool,
    file_name: &'a str,
    content: &'a str,
) -> Vec<String> {
    if flags.contains(&Flag::PrintFileNameOnly) {
        return match has_match(pattern, flags, content) {
            true => vec![String::from(file_name)],
            false => vec![],
        };
    }

    content
        .split("\n")
        .enumerate()
        .filter(|(_, line)| has_match(pattern, flags, line))
        .map(|(index, line)| {
            let line_match = LineMatch {
                line,
                file_name,
                line_number: index + 1,
            };
            format_match(flags, include_file_name, &line_match)
        })
        .filter(|result| !result.is_empty())
        .collect::<Vec<_>>()
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
