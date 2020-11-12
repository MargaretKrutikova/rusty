use std::fmt::Display;
use std::ops::{Add, Rem};

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    match_fn: fn(T) -> bool,
    substitute: &'static str,
}

impl<T> Matcher<T> {
    pub fn new(match_fn: fn(T) -> bool, substitute: &'static str) -> Matcher<T> {
        Matcher {
            match_fn,
            substitute,
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: 'static + Display + Add<Output = T> + Rem<Output = T> + From<u8> + PartialEq + Copy,
{
    pub fn is_divisible_by(element: &T, number: u8) -> bool {
        let result = *element % number.into();
        result == 0.into()
    }
    fn run_matchers(matchers: &Vec<Matcher<T>>, element: T) -> Option<String> {
        matchers.iter().fold(None, |acc, matcher| {
            match (acc, (matcher.match_fn)(element)) {
                (Some(acc_val), true) => Some(format!("{}{}", acc_val, matcher.substitute)),
                (Some(acc_val), false) => Some(acc_val),
                (None, true) => Some(String::from(matcher.substitute)),
                (None, false) => None,
            }
        })
    }

    pub fn new() -> Self {
        Fizzy {
            matchers: Vec::new(),
        }
    }

    pub fn add_matcher(self, matcher: Matcher<T>) -> Self {
        let mut matchers = self.matchers;
        matchers.push(matcher);
        Fizzy { matchers }
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        let matchers = self.matchers;

        iter.map(move |n| match Fizzy::run_matchers(&matchers, n) {
            None => n.to_string(),
            Some(val) => val,
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: 'static + Display + Add<Output = T> + Rem<Output = T> + From<u8> + PartialEq + Copy,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n| Fizzy::is_divisible_by(&n, 3), "fizz"))
        .add_matcher(Matcher::new(|n| Fizzy::is_divisible_by(&n, 5), "buzz"))
}
