use lazy_static::lazy_static;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    pub static ref STRATAGEMS: Vec<Stratagem> = vec![
        Stratagem::new("500 KG Bomb", vec!['w', 'a', 's', 's', 's']),
        Stratagem::new("Reinforce", vec!['w', 's', 'd', 'a', 'w']),
        Stratagem::new("Quasar Cannon", vec!['s', 's', 'w', 'a', 'd']),
        Stratagem::new("Resupply", vec!['s', 's', 'w', 'd']),
        Stratagem::new("Expendable Anti-Tank", vec!['s', 's', 'a', 'w', 'd']),
        Stratagem::new("MG-43 Machine Gun", vec!['d', 'a', 'd', 'w', 'd']),
        Stratagem::new("Gattling Century", vec!['s', 'w', 'd', 'a']),
    ];
}

#[derive(Error, Debug)]
pub enum StratError {
    #[error("There was an error when you put in the count (-c)")]
    InputCountError(#[from] std::num::ParseIntError),

    #[error("There was an error when parsing a string into a Stratagem ({0})")]
    ParseIntoStratagemError(String),

    #[error("There was an error with input/output")]
    IOError(#[from] std::io::Error),

    #[error("There was an error when generating a random number")]
    RandError,
}

#[derive(Debug, Clone)]
pub struct Stratagem {
    name: String,
    code: Vec<char>,
}

impl Stratagem {
    #[must_use]
    pub fn new(name: &str, code: Vec<char>) -> Self {
        Self {
            name: name.into(),
            code,
        }
    }

    #[must_use]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[must_use]
    pub fn get_code(&self) -> Vec<char> {
        self.code.clone()
    }
}

impl FromStr for Stratagem {
    type Err = StratError;
    fn from_str(inp: &str) -> Result<Self, StratError> {
        STRATAGEMS
            .iter()
            .filter(|strat| strat.get_name() == inp)
            .nth(0)
            .cloned()
            .ok_or_else(|| StratError::ParseIntoStratagemError(inp.to_string()))
    }
}
