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
    #[arg(long, default_value_t = false)]
    upper: bool,

    /// Include lowercase letters (default: true)
    #[arg(long, default_value_t = false)]
    lower: bool,

    /// Include digits (default: true)
    #[arg(long, default_value_t = false)]
    digits: bool,

    /// Include symbols (default: true)
    #[arg(long, default_value_t = false)]
    symbols: bool,

    /// Exclude character types
    #[arg(long, short = 'e')]
    exclude: Vec<char>,

    /// Number of passwords to generate (default: 1)
    #[arg(short, long, default_value_t = 1)]
    count: usize,
}

/// Generate cryptographically secure passwords with customizable character sets
/// 
/// This function uses OsRng as the seed for ChaCha8Rng to ensure cryptographic security.
/// It builds the character set dynamically based on the enabled options and excludes
/// any specified characters. The secure random number generator is then used to
/// select characters from the available set to construct the final password.
/// 
/// # Arguments
/// * `args` - Parsed command line arguments
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Use OsRng as seed for ChaCha8Rng to ensure cryptographic security
    let seed = rand::rngs::OsRng.gen();
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    let mut all_chars = String::new();

    // Build character set based on enabled options, excluding specified characters
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

    // Validate that at least one character type is selected
    if all_chars.is_empty() {
        eprintln!("Error: No character types selected.");
        std::process::exit(1);
    }

    // Generate the specified number of passwords
    for _ in 0..args.count {
        let password: String = (0..args.length)
            .map(|_| {
                // Securely select a random character from the available set
                let idx = rng.gen_range(0..all_chars.len());
                all_chars.chars().nth(idx).unwrap()
            })
            .collect();

        println!("{}", password);
    }

    Ok(())
}
