mod errors;
mod charsets;

use charsets::{Alpha, Digit, Special};
use rand::Rng;
use structopt::StructOpt;

use crate::charsets::CollatingSeq;

/// PassGen is a tool that lets you generate pseudo-random passwords from
/// the command line.
#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short, long, default_value="20")]
    length: usize,
    #[structopt(short, long, default_value="*")]
    alpha: Alpha,
    #[structopt(short, long, default_value="*")]
    digit: Digit,
    #[structopt(short, long, default_value="*")]
    special: Special,
}

impl Args {
    pub fn generate_charset(&self) -> Vec<char> {
        let mut chars = vec![];
        
        self.alpha.characters().iter().copied().for_each(|c|   chars.push(c));
        self.digit.characters().iter().copied().for_each(|c|   chars.push(c));
        self.special.characters().iter().copied().for_each(|c| chars.push(c));

        chars
    }
    pub fn generate_pass(&self) -> String {
        let charset = self.generate_charset();
        let mut out = String::new();
        for _ in 0..self.length {
            let idx = rand::thread_rng().gen_range(0..charset.len());
            out.push(charset[idx]);
        }
        out
    }
}

fn main() {
    println!("{}", Args::from_args().generate_pass());
}
