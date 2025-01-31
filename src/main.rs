use std::io::{self, BufRead, Write};

/// generate the hailstone sequince for a number
///
///     Vec<64> in function params is just for ease of recuresion
fn hailstone(hailstone_numbers: &Vec<u64>, n: u64) -> Vec<u64> {
    let mut hailstone_numbers: Vec<u64> = hailstone_numbers.to_vec();

    // adding the first number to the vector
    if hailstone_numbers.len() == 0 {
        hailstone_numbers.push(n);
    }

    // hailstone calculations
    match n % 2 {
        0 => {
            let n = n / 2;
            hailstone_numbers.push(n);
        }
        _ => {
            let n = n * 3 + 1;
            hailstone_numbers.push(n);
        }
    }

    // check if its 1 other wise enter recuresion and handle errors
    match &hailstone_numbers.last() {
        Some(1) => return hailstone_numbers,
        Some(n) => {
            return hailstone(&hailstone_numbers, **n);
        }
        None => panic!("failed to append the hailstone number to the vector."),
    }
}

fn main() {
    // prompt for user input
    print!("Calculate hailstone numbers enter starting number here \n > ");
    io::stdout().flush().unwrap(); // makes the '>' show while the user inputs

    // user input
    let mut user_buffer: String = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut user_buffer).unwrap(); // reading user buffer

    // parse the String into u64 with apopate error messages
    let n: u64 = match user_buffer.trim().parse::<u64>() {
        Ok(n) => n,
        Err(err) => panic!(
            "numbe entered must be a whole number and between [1, ..., 18446744073709551616] \n\nError: ('{err}')"
        ),
    };

    let hailstone_numbers: Vec<u64> = hailstone(&Vec::new(), n);

    println!("{hailstone_numbers:?}")
}
