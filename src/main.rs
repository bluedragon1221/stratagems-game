use clap::{arg, command, Parser};
use console::{style, Term};
use pretty_duration::pretty_duration;
use rand::{seq::SliceRandom, thread_rng};
use std::{
    io::{self, Write},
    str::FromStr,
    time::Instant,
};

use stratagems::{StratError, Stratagem, STRATAGEMS};

fn do_one(term: &Term, stratagem: &Stratagem) -> Result<(), StratError> {
    println!("{} ({:?})", stratagem.get_name(), stratagem.get_code());

    let start = Instant::now();
    for item in &stratagem.get_code() {
        let char = term.read_char()?;
        if char == *item {
            print!("{}", style("● ").green());
        } else {
            print!("{}", style("● ").red());
        }
        io::stdout().flush()?;
    }
    println!(
        "\nDone in {}",
        style(pretty_duration(&start.elapsed(), None)).yellow()
    );

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value_t = 5)]
    count: i32,

    // Specify the list to practice instead of practicing all of them
    #[arg(short, long, num_args = 1..)]
    practice_set: Option<Vec<String>>,
}

fn main() -> Result<(), StratError> {
    let term = Term::stdout();
    let args = Args::parse();

    let practice_set: Vec<Stratagem> = args.practice_set.map_or_else(
        || Ok(STRATAGEMS.to_vec()),
        |a| a.into_iter().map(|b| Stratagem::from_str(&b)).collect(),
    )?;

    for _ in 0..args.count {
        do_one(
            &term,
            practice_set
                .choose(&mut thread_rng())
                .ok_or(StratError::RandError)?,
        )?;
    }

    Ok(())
}
