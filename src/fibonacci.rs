use num::traits::Pow;
use rprompt::prompt_reply;
use std::error::Error;
use std::fmt::Debug;
use std::io;
use std::str::FromStr;
use std::time::Instant;

pub struct SafeInput<T> {
    value: T,
}

impl<T> SafeInput<T>
where
    T: TryFrom<u32> + Copy + Into<u32> + FromStr + Debug,
{
    pub fn new(value: T) -> Result<Self, Box<dyn Error>> {
        if Self::validate(&value) {
            Ok(Self { value })
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Input!").into())
        }
    }

    pub fn new_via_prompt(message: &str) -> Result<Self, Box<dyn Error>> {
        Self::new(prompt_reply(message)?
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, "Invalid Input!"))?)
    }

    fn validate(p0: &T) -> bool {
        let value = match u32::try_from(*p0) {
            Ok(v) => v,
            Err(_) => return false,
        };
        value > 0
    }

    fn value(&self) -> T {
        self.value
    }
}

// Usual recursive implementation
fn fibo_r(n: u32) -> u64 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fibo_r(n - 1) + fibo_r(n - 2),
    }
}

// Usual iterative implementation
fn fibo_i(n: u32) -> u64 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fibo_iter(n),
    }
}

fn fibo_iter(n: u32) -> u64 {
    let mut fin: u64 = 0;
    let mut fin1: u64 = 1;
    let mut fin2: u64 = 1;
    for _i in 3..=n {
        fin = fin1 + fin2;
        fin2 = fin1;
        fin1 = fin;
    }
    fin
}

fn fibo_c(n: u32) -> u64 {
    let au: f64 = (1_f64 + 5_f64.sqrt()) / 2_f64;
    (au.pow(n as f64) / 5_f64.sqrt()).round() as u64
}

pub fn main() {
    let n: u32 = SafeInput::new_via_prompt("Input n for F_n: ").map_or_else(
        |_| {
            println!("Negative input! Forcing to 0 ");
            0
        },
        |t| t.value(),
    );
    let start = Instant::now();
    println!("Fc_{n} = {} took {:?}", fibo_c(n), start.elapsed());

    let start = Instant::now();
    println!("Fi_{n} = {} took {:?}", fibo_i(n), start.elapsed());

    let start = Instant::now();
    println!("Fr_{n} = {} took {:?}", fibo_r(n), start.elapsed());
}
