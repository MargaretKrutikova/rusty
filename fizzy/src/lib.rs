use std::fmt::Display;
use std::ops::{Add, Rem};

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    run_fn: Box<dyn Fn(T) -> bool>,
    substitute: &'static str,
}

impl<T> Matcher<T> {
    pub fn new(matcher: impl Fn(T) -> bool + 'static, substitute: &'static str) -> Matcher<T> {
        Matcher {
            run_fn: Box::new(matcher),
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
    T: Display + Add<Output = T> + Rem<Output = T> + From<u8> + PartialEq + Copy,
{
    pub fn is_divisible_by(element: &T, number: u8) -> bool {
        let result = *element % number.into();
        result == 0.into()
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
        iter.map(|n| {
            let result = self
                .matchers
                .iter()
                .map(|matcher| match (matcher.run_fn)(n) {
                    true => matcher.substitute,
                    false => "",
                })
                .collect::<Vec<&'static str>>()
                .join(" ");
            match result.is_empty() {
                true => n.to_string(),
                false => result,
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Display + Add<Output = T> + Rem<Output = T> + From<u8> + PartialEq + Copy,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n| Fizzy::is_divisible_by(&n, 3), "fizz"))
        .add_matcher(Matcher::new(|n| Fizzy::is_divisible_by(&n, 5), "buzz"))
}
