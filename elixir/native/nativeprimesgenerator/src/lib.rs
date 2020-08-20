use rustler::{Encoder, Env, Error, Term};
use serde::{Deserialize, Serialize};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler::rustler_export_nifs! {
    "Elixir.NativePrimesGenerator",
    [
        ("generate_primes", 1, generate_primes)
    ],
    None
}

#[derive(Debug, Serialize, Deserialize)]
struct PrimesResult {
    primes: Vec<i32>,
}

fn generate_primes<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let limit: i32 = args[0].decode()?;
    let mut result = PrimesResult { primes: Vec::new() };
    let mut counter = 0;
    let mut current_value = 0;

    while counter <= limit {
        if is_prime(current_value) {
            result.primes.push(current_value);
        }
        current_value += 1;
        counter += 1;
    }

    Ok((atoms::ok(), serde_json::to_string(&result).unwrap()).encode(env))
}

fn is_prime(n: i32) -> bool {
    match n <= 1 {
        true => false,
        false => {
            for v in 2..n {
                if n % v == 0 {
                    return false;
                }
            }

            true
        }
    }
}
