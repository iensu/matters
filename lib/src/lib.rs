use std::{collections::HashSet, fmt::Display, str::FromStr};

use error::{ErrorKind, LibError};
use rand::Rng;

pub mod error;

type Result<T> = std::result::Result<T, error::LibError>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "ffi", repr(u32))]
pub enum Operation {
    #[cfg_attr(feature = "clap", clap(name = "+"))]
    Addition,
    #[cfg_attr(feature = "clap", clap(name = "-"))]
    Subtraction,
    #[cfg_attr(feature = "clap", clap(name = "*"))]
    Multiplication,
    #[cfg_attr(feature = "clap", clap(name = "/"))]
    Division,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Addition => write!(f, "+"),
            Self::Subtraction => write!(f, "-"),
            Self::Multiplication => write!(f, "*"),
            Self::Division => write!(f, "/"),
        }
    }
}

impl FromStr for Operation {
    type Err = LibError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Addition),
            "-" => Ok(Self::Subtraction),
            "*" => Ok(Self::Multiplication),
            "/" => Ok(Self::Division),
            _ => Err(LibError {
                kind: ErrorKind::InvalidInput,
                message: format!("Not a supported operation: {s}"),
            }),
        }
    }
}

impl Operation {
    /// Generates a problem based on the operation.
    ///
    /// # Errors
    ///
    /// Throws an error if unable to generate problem.
    pub fn generate_problem(&self, options: &Options, rng: &mut impl Rng) -> Result<Problem> {
        let (x, y) = match self {
            Self::Addition => {
                let x = rng.gen_range(1..options.max);
                let y = rng.gen_range(1..=(options.max - x));

                (x, y)
            }
            Self::Subtraction => {
                let x = rng.gen_range(1..=options.max);
                let y = rng.gen_range(1..=options.max);

                if !options.allow_negative && x < y {
                    (y, x)
                } else {
                    (x, y)
                }
            }
            Self::Multiplication => {
                let x = rng.gen_range(0..=options.max_factor);
                let y = rng.gen_range(0..=options.max_factor);
                (x, y)
            }
            Self::Division => {
                let divisor = rng.gen_range(1..=options.max_factor);
                let dividends: Vec<u32> = (0..=(options.max_factor * options.max_factor))
                    .filter(|dividend| dividend % divisor == 0 && *dividend <= divisor * 12)
                    .collect();
                let distribution =
                    rand::distributions::Slice::new(&dividends).map_err(|_| LibError {
                        kind: ErrorKind::ProblemGenerationDivision,
                        message: format!(
                            "Failed to create a sample distribution from {dividends:?}"
                        ),
                    })?;
                let dividend = rng.sample(distribution);

                (*dividend, divisor)
            }
        };

        Ok(Problem::new(*self, x, y))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Problem {
    pub operation: Operation,
    pub x: u32,
    pub y: u32,
}

impl Problem {
    #[must_use]
    pub const fn new(operation: Operation, x: u32, y: u32) -> Self {
        Self { operation, x, y }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>2} {} {:>2}", self.x, self.operation, self.y)
    }
}

/// Problem generation options.
#[derive(Debug)]
pub struct Options {
    /// Number of problems.
    pub problems: usize,
    /// Max result in addition problems, and max terms in subtraction problems.
    pub max: u32,
    /// Max factor in multiplication problems, and the max divisor in division problems
    pub max_factor: u32,
    /// The kind of operations to generate problems for.
    pub operations: Vec<Operation>,
    /// Allow negative results in subtraction problems.
    pub allow_negative: bool,
}

/// Generates a list of unique math problems.
///
/// # Errors
///
/// Returns an error if it fails to create a list of problems.
pub fn generate_problems(options: &Options, rng: &mut impl Rng) -> Result<Vec<Problem>> {
    let mut problems: HashSet<Problem> = HashSet::new();

    let mut iterations = 0;
    let operations =
        rand::distributions::Slice::new(&options.operations).map_err(|_| LibError {
            kind: ErrorKind::ProblemGeneration,
            message: format!(
                "Failed to generate operation distribution from {:?}",
                options.operations
            ),
        })?;

    while problems.len() < options.problems && iterations < 10_000 {
        let operation = rng.sample(operations);
        let problem = operation.generate_problem(options, rng)?;
        problems.insert(problem);
        iterations += 1;
    }

    Ok(problems.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn operator_parsing_works() {
        assert_eq!(Operation::from_str("+").unwrap(), Operation::Addition);
        assert_eq!(Operation::from_str("-").unwrap(), Operation::Subtraction);
        assert_eq!(Operation::from_str("*").unwrap(), Operation::Multiplication);
        assert_eq!(Operation::from_str("/").unwrap(), Operation::Division);

        assert!(Operation::from_str("foo").is_err());
    }

    #[test]
    fn problem_equality_works() {
        let problem_a = Problem::new(Operation::Addition, 22, 20);
        let problem_b = Problem::new(Operation::Addition, 22, 20);
        let problem_c = Problem::new(Operation::Subtraction, 22, 20);

        assert!(problem_a == problem_b);
        assert!(problem_a != problem_c);
    }

    #[test]
    fn problem_printing_works() {
        assert_eq!(
            &Problem::new(Operation::Multiplication, 5, 7).to_string(),
            " 5 *  7"
        );
        assert_eq!(
            &Problem::new(Operation::Subtraction, 12, 6).to_string(),
            "12 -  6"
        );
        assert_eq!(
            &Problem::new(Operation::Addition, 22, 20).to_string(),
            "22 + 20"
        );
        assert_eq!(
            &Problem::new(Operation::Division, 2, 1).to_string(),
            " 2 /  1"
        );
    }
}
