use std::{env::args, process::exit, num::IntErrorKind, time::{Duration, Instant}};

// Print factors and time elapsed
fn print_results(factors: &Vec<u64>, duration: Option<Duration>) {
    if let Some(time) = duration { if time.as_secs() > 0 { print!("[{}.{}s] ", time.as_secs(), time.subsec_millis()/10); } }
    else { print!("Found: "); };

    if factors.len() > 0 { for factor in factors { print!("{} ", factor); } }
    else if let Some(_) = duration { print!("Prime: 1"); }
    else { print!("None"); };
    println!();
}

// Find factors and duration
fn factor_arg(mut arg: u64) -> (Vec<u64>, Duration) {
    let mut factors: Vec<u64> = vec![];

    let now = Instant::now();

    for i in 2..=arg/2 {       
        if now.elapsed().as_secs() >= 100 { println!("Timeout: Running for 100s"); print_results(&factors, None); exit(3) }
        if i > arg { break };
        while arg % i == 0 { arg /= i; factors.push(i); };
    };

    (factors,  now.elapsed())
}

// Parse input to number
fn parse_arg(mut arg: String) -> u64 {
    arg = if arg.contains('-') { arg.replace('-', "") } else { arg };
    arg = if arg.contains(',') { arg.replace(',', "") } else { arg };
    arg = if arg.contains('.') { arg.replace('.', "") } else { arg };
    arg = if arg.contains('_') { arg.replace('_', "") } else { arg };

    match arg.parse() {
        Ok(num) => num,
        Err(err) => {
            match err.kind() {
                IntErrorKind::InvalidDigit => { eprintln!("Error: Only use (0-9), comma, underscore, or period"); exit(2); },
                IntErrorKind::PosOverflow => { eprintln!("Error: Integer too large\nMaxInt: {}", u64::MAX); exit(2); }
                IntErrorKind::Empty => { eprintln!("Error: Must include numbers (0-9)"); exit(2); }
                _ => { eprintln!("Error: {}", err); exit(2); }
            }
        }
    }
}

// Get input as string
fn get_input() -> String {
    match args().nth(1) {
        Some(arg) => arg,
        None => { println!("Usage: COMMAND <number>"); exit(1); }
    }
}

// Factor the given number
fn main() {
    let arg = get_input();
    // println!("String: {}", arg_str);

    let arg = parse_arg(arg);
    // println!("u64: {}", arg);

    let (factors, duration) = factor_arg(arg);
    // println!("{:?}", factors);

    print_results(&factors, Some(duration));
}
