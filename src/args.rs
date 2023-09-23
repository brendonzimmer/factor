use std::{
    env, fmt,
    num::{IntErrorKind, NonZeroU64},
    time::Duration,
};

#[derive(Debug)]
pub enum InputError {
    Help, // can make this a long help
    BadInput,
    InvalidNumber(IntErrorKind),
    InvalidTimeout(IntErrorKind),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Help => write!(f, "Usage: COMMAND <INTEGER> [-t <SECONDS>]"),
            Self::BadInput => write!(f, "Usage: COMMAND <INTEGER> [-t <SECONDS>]"),
            Self::InvalidNumber(IntErrorKind::InvalidDigit) => {
                write!(f, "InvalidNumber: Only use (0..=9) and comma OR underscore")
            }
            Self::InvalidNumber(IntErrorKind::PosOverflow) => {
                write!(f, "InvalidNumber: Too large\nMaxInt: {}", u64::MAX)
            }
            Self::InvalidNumber(IntErrorKind::Empty) => {
                write!(f, "InvalidNumber: Must include numbers (0..=9)")
            }
            Self::InvalidTimeout(IntErrorKind::InvalidDigit) => {
                write!(f, "InvalidTimeout: Only use (0..=9)")
            }
            Self::InvalidTimeout(IntErrorKind::PosOverflow) => {
                write!(f, "InvalidTimeout: Too large\nMaxInt: {}", u64::MAX)
            }
            Self::InvalidTimeout(IntErrorKind::Empty) => {
                write!(f, "InvalidTimeout: Remove the -t flag to disable timeout")
            }
            Self::InvalidTimeout(IntErrorKind::Zero) => write!(
                f,
                "InvalidTimeout: Timeout cannot be 0; to disable timeout, remove the -t flag"
            ),
            Self::InvalidNumber(_) | Self::InvalidTimeout(_) => {
                write!(f, "ParseFailure: Unknown error")
            }
        }
    }
}

impl std::error::Error for InputError {}

type Args = (u64, Option<Duration>);

/// Parses the command line arguments.
pub fn parse() -> Result<Args, InputError> {
    // println!("args: {:?}", env::args().collect::<Vec<String>>());
    if env::args()
        .find(|arg| arg.eq("-h") || arg.eq("--help"))
        .is_some()
    {
        return Err(InputError::Help);
    }

    let len = env::args().len();
    if len <= 1 || len == 3 || len >= 5 {
        return Err(InputError::BadInput);
    }

    let args: Vec<Option<String>> = env::args()
        .enumerate()
        .map(|(i, arg)| {
            if i == 1 {
                if arg.contains(',') {
                    return Some(arg.replace(',', ""));
                }
                if arg.contains('_') {
                    return Some(arg.replace('_', ""));
                }
                return Some(arg);
            }
            if i == 2 {
                return arg.eq("-t").then_some(arg);
            }
            if i == 3 {
                return Some(arg);
            }
            return None;
        })
        .collect();

    let num = match args[1].as_ref().unwrap().trim().parse::<u64>() {
        Ok(num) => num,
        Err(err) => return Err(InputError::InvalidNumber(err.kind().to_owned())),
    };

    if len == 2 {
        return Ok((num, None));
    }

    if len == 4 && args[2].is_none() {
        return Err(InputError::BadInput); // "Usage: COMMAND <INTEGER> [-t <SECONDS>]"
    }

    let timeout: u64 = match args[3].as_ref().unwrap().trim().parse::<NonZeroU64>() {
        Ok(num) => num.into(),
        Err(err) => return Err(InputError::InvalidTimeout(err.kind().to_owned())),
    };

    Ok((num, Some(Duration::from_secs(timeout))))
}
