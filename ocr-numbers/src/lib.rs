// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

fn validate_rows(input: &str) -> Option<Error> {
    let rows = input.split("\n").collect::<Vec<_>>();
    match rows.len() % 4 == 0 {
        true => None,
        false => Some(Error::InvalidRowCount(rows.len())),
    }
}

fn validate_cols(input: &str) -> Option<Error> {
    input.split("\n").fold(None, |result, row| {
        result.or_else(|| match row.len() % 3 == 0 {
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
        None => Ok(String::from("")),
        Some(err) => Err(err),
    }
}
