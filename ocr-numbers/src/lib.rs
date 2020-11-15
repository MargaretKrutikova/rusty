// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

const ROWS: usize = 4;
const COLS: usize = 3;

fn to_number(s: &String) -> String {
    #[rustfmt::skip]
    let numbers = [
        " _ \n".to_string() +
        "| |\n" +
        "|_|\n" +
        "   ",
        "   \n".to_string() +
        "  |\n" +
        "  |\n" +
        "   ",
        " _ \n".to_string() +
        " _|\n" +
        "|_ \n" +
        "   ",
        " _ \n".to_string() +
        " _|\n" +
        " _|\n" +
        "   ",
        "   \n".to_string() +
        "|_|\n" +
        "  |\n" +
        "   ",
        " _ \n".to_string() +
        "|_ \n" +
        " _|\n" +
        "   ",
        " _ \n".to_string() +
        "|_ \n" +
        "|_|\n" +
        "   ",
        " _ \n".to_string() +
        "  |\n" +
        "  |\n" +
        "   ",
        " _ \n".to_string() +
        "|_|\n" +
        "|_|\n" +
        "   ",
        " _ \n".to_string() +
        "|_|\n" +
        " _|\n" +
        "   "
    ];
    let maybe_position = numbers.iter().position(|r| r == s);
    match maybe_position {
        Some(position) => position.to_string(),
        None => String::from("?"),
    }
}

fn transform_input(input: &str) -> String {
    let rows = input
        .split("\n")
        .map(|row| row.split("\n").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let number_of_letters = rows[0].len();
    println!("{}", number_of_letters);

    (0..number_of_letters)
        .map(|index| {
            let f = rows
                .iter()
                .map(|cols| cols[index])
                .collect::<Vec<_>>()
                .join("\n");
            to_number(&f)
        })
        .collect::<Vec<_>>()
        .join("")
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
