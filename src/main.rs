use clap::Parser;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Parser, Debug)]
#[command(name = "password-generator")]
#[command(author = "Duskthoth")]
#[command(version = "0.1.0")]
#[command(about = "Generate cryptographically secure passwords")]
struct Args {
    /// Password length (default: 12)
    #[arg(short, long, default_value_t = 12)]
    length: usize,

    /// Include uppercase letters (default: true)
    #[arg(long, default_value_t = true)]
    upper: bool,

    /// Include lowercase letters (default: true)
    #[arg(long, default_value_t = true)]
    lower: bool,

    /// Include digits (default: true)
    #[arg(long, default_value_t = true)]
    digits: bool,

    /// Include symbols (default: true)
    #[arg(long, default_value_t = true)]
    symbols: bool,

    /// Exclude character types
    #[arg(long, short = 'e')]
    exclude: Vec<char>,

    /// Number of passwords to generate (default: 1)
    #[arg(short, long, default_value_t = 1)]
    count: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Use OsRng as seed for ChaCha8Rng
    let seed = rand::rngs::OsRng.gen();
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    let mut all_chars = String::new();

    if args.upper {
        for c in 'A'..='Z' {
            if !args.exclude.contains(&c) {
                all_chars.push(c);
            }
        }
    }

    if args.lower {
        for c in 'a'..='z' {
            if !args.exclude.contains(&c) {
                all_chars.push(c);
            }
        }
    }

    if args.digits {
        for c in '0'..='9' {
            if !args.exclude.contains(&c) {
                all_chars.push(c);
            }
        }
    }

    if args.symbols {
        let sym = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
        for c in sym.chars() {
            if !args.exclude.contains(&c) {
                all_chars.push(c);
            }
        }
    }

    if all_chars.is_empty() {
        eprintln!("Error: No character types selected.");
        std::process::exit(1);
    }

    for _ in 0..args.count {
        let password: String = (0..args.length)
            .map(|_| {
                let idx = rng.gen_range(0..all_chars.len());
                all_chars.chars().nth(idx).unwrap()
            })
            .collect();

        println!("{}", password);
    }

    Ok(())
}
