// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

const ROWS: usize = 4;
const COLS: usize = 3;

#[rustfmt::skip]
const NUMBERS: [&str; 10]= [
    concat!(
    " _ \n",
    "| |\n",
    "|_|\n",
    "   "),
    concat!(
    "   \n",
    "  |\n",
    "  |\n",
    "   "),
    concat!(
    " _ \n",
    " _|\n" ,
    "|_ \n" ,
    "   "),
    concat!(
    " _ \n",
    " _|\n",
    " _|\n",
    "   "),
    concat!(
    "   \n",
    "|_|\n",
    "  |\n",
    "   "),
    concat!(
    " _ \n",
    "|_ \n",
    " _|\n",
    "   "),
    concat!(
    " _ \n",
    "|_ \n",
    "|_|\n",
    "   "),
    concat!(
    " _ \n",
    "  |\n",
    "  |\n",
    "   "),
    concat!(
    " _ \n",
    "|_|\n",
    "|_|\n",
    "   "),
    concat!(
    " _ \n",
    "|_|\n" ,
    " _|\n",
    "   ")
];

fn to_number(s: &str) -> String {
    let maybe_position = NUMBERS.iter().position(|number_str| *number_str == s);
    match maybe_position {
        Some(position) => position.to_string(),
        None => String::from("?"),
    }
}

fn extract_letter_by_ind(row: &[&str], letter_ind: usize) -> String {
    let start_position = letter_ind * COLS;
    row.iter()
        .map(|cols| &cols[start_position..start_position + COLS])
        .collect::<Vec<&str>>()
        .join("\n")
}

fn transform_row(row: &[&str]) -> String {
    let number_of_letters = row[0].len() / COLS;

    (0..number_of_letters)
        .map(|index| {
            let letter = extract_letter_by_ind(row, index);
            to_number(&letter)
        })
        .collect::<Vec<_>>()
        .join("")
}

fn transform_input(input: &str) -> String {
    let rows = input.split("\n").collect::<Vec<_>>();
    let number_of_rows = rows.len() / ROWS;

    (0..number_of_rows)
        .map(|index| transform_row(&rows[index * ROWS..index * ROWS + ROWS]))
        .collect::<Vec<_>>()
        .join(",")
}

fn validate_rows(input: &str) -> Option<Error> {
    let rows = input.split("\n").collect::<Vec<_>>();
    match rows.len() % ROWS == 0 {
        true => None,
        false => Some(Error::InvalidRowCount(rows.len())),
    }
}

fn validate_cols(input: &str) -> Option<Error> {
    input.split("\n").fold(None, |result, row| {
        result.or_else(|| match row.len() % COLS == 0 {
            true => None,
            false => Some(Error::InvalidColumnCount(row.len())),
        })
    })
}

fn validate_input(input: &str) -> Option<Error> {
    validate_rows(input).or_else(|| validate_cols(input))
}

pub fn convert(input: &str) -> Result<String, Error> {
    match validate_input(input) {
        None => Ok(transform_input(input)),
        Some(err) => Err(err),
    }
}
