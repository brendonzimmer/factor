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
pub enum FactorError {
    NeitherPrimeNorComposite(u64),
}

impl std::fmt::Display for FactorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactorError::NeitherPrimeNorComposite(n) => {
                write!(f, "InvalidNumber: {} is neither prime nor composite", n)
            }
        }
    }
}

/// Find prime factors of a number using trial division
pub fn trial_division(mut n: u64) -> Result<Factor, FactorError> {
    if n == 0 || n == 1 {
        return Err(FactorError::NeitherPrimeNorComposite(n));
    }

    if n == 2 {
        return Ok(Factor::Prime(n));
    }

    let mut list: FactorList = vec![];
    while (n & 1) == 0 {
        n >>= 1;
        list.push(2);
    }

    let mut f: u64 = 3;
    while f.checked_pow(2).map_or(false, |x| x <= n) {
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