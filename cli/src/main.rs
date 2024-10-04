use std::{fs::File, io::Write};

use clap::Parser;
use matters_lib::{generate_problems, Operation, Options};

#[derive(Debug, Parser)]
#[command(name = "matte", version, about, long_about = None)]
struct Args {
    /// Total number of problems
    #[arg(short, long, default_value_t = 60)]
    problems: usize,

    /// Max result in addition problems, and max terms in subtraction problems.
    #[arg(short, long, default_value_t = 20)]
    max: u32,

    /// Max factor in multiplication problems, and the max divisor in division problems.
    #[arg(short = 'f', long, default_value_t = 10)]
    max_factor: u32,

    /// The kind of operations to generate problems for.
    #[arg(short, long, value_enum, default_values_t = vec![Operation::Addition, Operation::Subtraction, Operation::Multiplication], value_delimiter = ',')]
    operations: Vec<Operation>,

    /// Allow negative results in subtraction problems.
    #[arg(long, default_value_t = false)]
    allow_negative: bool,

    /// Name of the generated PDF file.
    #[arg(short, short, default_value = "test.pdf")]
    name: String,
}

impl From<Args> for Options {
    fn from(value: Args) -> Self {
        Self {
            problems: value.problems,
            max: value.max,
            max_factor: value.max_factor,
            operations: value.operations,
            allow_negative: value.allow_negative,
        }
    }
}

fn main() {
    let args = Args::parse();

    let filename = args.name.clone();
    let mut rng = rand::thread_rng();
    let problems = generate_problems(&args.into(), &mut rng).unwrap();

    let bytes = matters_pdf::generate_pdf(&problems);

    let mut file = File::create(&filename).expect("can create file");
    file.write_all(&bytes).expect("can write data to file");
}
