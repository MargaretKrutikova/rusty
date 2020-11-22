use anyhow::Error;
use std::fs;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug)]
pub struct Flags {
    case_insensitive: bool,
    print_line_numbers: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags {
            case_insensitive: flags.iter().any(|flag| *flag == "-i"),
            print_line_numbers: flags.iter().any(|flag| *flag == "-n"),
        }
    }
}

fn contains(pattern: &str, flags: &Flags, line: &str) -> bool {
    if flags.case_insensitive {
        let p = pattern.to_lowercase();
        line.to_lowercase().contains(&p)
    } else {
        line.contains(pattern)
    }
}

fn display_line(flags: &Flags, (index, line): (usize, &str)) -> String {
    if flags.print_line_numbers {
        format!("{}:{}", index + 1, line)
    } else {
        String::from(line)
    }
}

fn grep_file(pattern: &str, flags: &Flags, file: &str) -> Result<Vec<String>, Error> {
    let matches = fs::read_to_string(file)?
        .split("\n")
        .enumerate()
        .filter(|(_, line)| contains(pattern, flags, line))
        .map(|pair| display_line(flags, pair))
        .collect::<Vec<String>>();

    Ok(matches)
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut matching_lines: Vec<String> = Vec::new();
    for file in files.iter() {
        matching_lines.extend(grep_file(pattern, flags, file)?);
    }
    Ok(matching_lines)
}
