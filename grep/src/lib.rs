use anyhow::Error;
use std::fs;

#[derive(Debug)]
pub struct Flags {
    print_line_number: bool,
    print_file_name_only: bool,
    case_insensitive: bool,
    invert_result: bool,
    match_entire_line: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        flags.iter().fold(Flags::default_flags(), |mut acc, flag| {
            match *flag {
                "-n" => acc.print_line_number = true,
                "-l" => acc.print_file_name_only = true,
                "-i" => acc.case_insensitive = true,
                "-v" => acc.invert_result = true,
                "-x" => acc.match_entire_line = true,
                _ => (),
            }
            acc
        })
    }
    fn default_flags() -> Self {
        Flags {
            print_file_name_only: false,
            print_line_number: false,
            case_insensitive: false,
            match_entire_line: false,
            invert_result: false,
        }
    }
}

fn match_text(pattern: &str, flags: &Flags, text: &str) -> bool {
    if flags.match_entire_line {
        text == pattern
    } else {
        text.contains(pattern)
    }
}

fn has_match(pattern: &str, flags: &Flags, line: &str) -> bool {
    let matches_pattern = if flags.case_insensitive {
        match_text(&pattern.to_lowercase(), flags, &line.to_lowercase())
    } else {
        match_text(pattern, flags, line)
    };

    match flags.invert_result {
        false => matches_pattern,
        true if !line.is_empty() => !matches_pattern,
        _ => false,
    }
}

fn format_match(
    flags: &Flags,
    include_file_name: bool,
    file_name: &str,
    (index, line): (usize, &str),
) -> String {
    let formatted_line = match flags.print_line_number {
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
    flags: &Flags,
    include_file_name: bool,
    file_name: &str,
    content: &str,
) -> Vec<String> {
    let mut matches = content
        .split("\n")
        .enumerate()
        .filter(|(_, line)| has_match(pattern, flags, line));

    match flags.print_file_name_only {
        true => matches
            .next()
            .map_or(vec![], |_| vec![String::from(file_name)]),
        false => matches
            .map(|pair| format_match(flags, include_file_name, file_name, pair))
            .filter(|result| !result.is_empty())
            .collect::<Vec<_>>(),
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut matching_lines: Vec<String> = Vec::new();
    let include_file_name = files.len() > 1;

    for file_name in files.iter() {
        let content = fs::read_to_string(file_name)?;
        let matches = match_file_content(pattern, &flags, include_file_name, file_name, &content);

        matching_lines.extend(matches);
    }
    Ok(matching_lines)
}
