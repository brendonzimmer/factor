use std::time::{Duration, Instant};

type FactorList = Vec<u64>;

#[derive(Debug)]
pub enum Factor {
    Prime(u64),
    Composite(FactorList),
}

impl std::fmt::Display for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Factor::Prime(prime) => write!(f, "Prime: {}", prime),
            Factor::Composite(factors) => {
                write!(f, "Found: ")?;
                for factor in factors {
                    write!(f, "{} ", factor)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    NeitherPrimeNorComposite(u64),
    Timeout(Option<FactorList>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NeitherPrimeNorComposite(n) => {
                write!(f, "InvalidNumber: {} is neither prime nor composite", n)
            }
            Error::Timeout(Some(factors)) => write!(f, "Timeout: Found {:?}", factors),
            Error::Timeout(None) => write!(f, "Timeout: No factors found"),
        }
    }
}

impl std::error::Error for Error {}

impl From<FactorList> for Error {
    fn from(factors: FactorList) -> Self {
        Error::Timeout(factors.len().gt(&0).then_some(factors))
    }
}

pub struct Params {
    n: u64,
    t: Option<Duration>,
}

impl From<(u64, Option<Duration>)> for Params {
    fn from((n, t): (u64, Option<Duration>)) -> Self {
        Self { n, t }
    }
}

/// Find prime factors of a number using trial division
pub fn trial_division(Params { mut n, t }: Params) -> Result<Factor, Error> {
    if n == 0 || n == 1 {
        return Err(Error::NeitherPrimeNorComposite(n));
    }

    if n == 2 {
        return Ok(Factor::Prime(n));
    }

    let now = Instant::now();
    let mut list: FactorList = vec![];

    while (n & 1) == 0 {
        n >>= 1;
        list.push(2);
    }

    let mut f: u64 = 3;
    while f.checked_pow(2).map_or(false, |x| x <= n) {
        if t.map_or(false, |d| now.elapsed() >= d) {
            return Err(list.into());
        }

        while (n % f) == 0 {
            list.push(f);
            n /= f;
        }
        f += 2;
    }

    if list.len() == 0 {
        return Ok(Factor::Prime(n));
    }
    
    if n != 1 {
        list.push(n);
    }

    Ok(Factor::Composite(list))
}
