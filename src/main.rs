mod args;
mod factor;

use args::parse;
use factor::{trial_division, Params};

// Factor the given number
fn main() {
    let args: Params = match parse() {
        Ok(args) => args.into(),
        Err(err) => return println!("{}", err),
    };

    match trial_division(args) {
        Ok(factors) => println!("{}", factors),
        Err(err) => println!("{}", err),
    }
}
