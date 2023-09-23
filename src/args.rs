use std::{
    env, fmt,
    num::IntErrorKind,
};

#[derive(Debug)]
pub enum InputError {
    Help, // can make this a long help
    BadInput,
    InvalidNumber(IntErrorKind),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Help | Self::BadInput => write!(f, "Usage: COMMAND <INTEGER>"),
            Self::InvalidNumber(IntErrorKind::InvalidDigit) => {
                write!(f, "InvalidNumber: Only use (0..=9) and comma OR underscore")
            }
            Self::InvalidNumber(IntErrorKind::PosOverflow) => {
                write!(f, "InvalidNumber: Too large\nMaxInt: {}", u64::MAX)
            }
            Self::InvalidNumber(IntErrorKind::Empty) => {
                write!(f, "InvalidNumber: Must include numbers (0..=9)")
            }
            Self::InvalidNumber(_) => {
                write!(f, "ParseFailure: Unknown error")
            }
        }
    }
}

impl std::error::Error for InputError {}

type Arg = u64;

/// Parses the command line arguments.
pub fn parse() -> Result<Arg, InputError> {
    // if help specified, return help
    if env::args()
        .find(|arg| arg.eq("-h") || arg.eq("--help"))
        .is_some()
    {
        return Err(InputError::Help);
    }

    // if no args or too many args, return bad input
    let len = env::args().len();
    if len <= 1 || len >= 3 {
        return Err(InputError::BadInput);
    }

    // get number to factor as string arg
    let mut arg: String = env::args().collect::<Vec<String>>()[1].clone();
    
    // remove commas or underscores
    if arg.contains(',') {
         arg = arg.replace(',', "");
    } 
    else if arg.contains('_') {
        arg = arg.replace('_', "");
    }

    // parse into number
    let num = match arg.trim().parse::<u64>() {
        Ok(num) => num,
        Err(err) => return Err(InputError::InvalidNumber(err.kind().to_owned())),
    };

    Ok(num)
}
