mod args;
mod factor;

use args::parse;
use factor::trial_division;

// Factor the given number
fn main() {
    let num = match parse() {
        Ok(num) => num,
        Err(err) => return println!("{}", err),
    };

    match trial_division(num) {
        Ok(factors) => println!("{}", factors),
        Err(err) => println!("{}", err),
    }
}
