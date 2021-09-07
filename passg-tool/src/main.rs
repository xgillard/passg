use passg_lib::prelude::*;
use structopt::StructOpt;

/// PassGen is a tool that lets you generate pseudo-random passwords from
/// the command line.
#[derive(Debug, StructOpt)]
struct Args {
    /// The length of the password
    #[structopt(short, long, default_value = "20")]
    length: usize,
    /// What kind of alphabetic characters do you want to allow ?
    /// (all = 'all', none = 'none', easily distinguished = 'dist' [eg removes O vs 0],
    /// lower case = 'lower', upper case = 'upper')
    #[structopt(short, long, default_value = "dist")]
    alpha: Alpha,
    /// What kind of numeric characters do you want to allow ?
    /// (all = 'all', none = 'none', easily distinguished = 'dist' [eg removes O vs 0])
    #[structopt(short, long, default_value = "dist")]
    digit: Digit,
    /// What kind of special characters do you want to allow ?
    /// (all = 'all', none = 'none', the most common ones = 'basic')
    #[structopt(short, long, default_value = "basic")]
    special: Special,
}

impl From<Args> for Generator {
    fn from(args: Args) -> Generator {
        GeneratorBuilder::default()
            .length(args.length)
            .alpha(args.alpha)
            .digit(args.digit)
            .special(args.special)
            .build()
            .expect("Could not build a password generator") // will not occur
    }
}

fn main() {
    let generator = Generator::from(Args::from_args());
    println!("{}", generator.generate());
}
