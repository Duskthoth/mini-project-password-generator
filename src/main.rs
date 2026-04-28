use clap::Parser;
use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;
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

    /// Include uppercase letters
    #[arg(long, default_value_t = false)]
    upper: bool,

    /// Include lowercase letters
    #[arg(long, default_value_t = false)]
    lower: bool,

    /// Include digits
    #[arg(long, default_value_t = false)]
    digits: bool,

    /// Include symbols
    #[arg(long, default_value_t = false)]
    symbols: bool,

    /// Exclude specific characters
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

    let mut char_groups: Vec<(&str, Vec<char>)> = Vec::new();

    // Build character set based on enabled options, excluding specified characters
    if args.upper {
        let chars: Vec<char> = ('A'..='Z')
            .filter(|c| !args.exclude.contains(c))
            .collect();
        char_groups.push(("uppercase", chars));
    }

    if args.lower {
        let chars: Vec<char> = ('a'..='z')
            .filter(|c| !args.exclude.contains(c))
            .collect();
        char_groups.push(("lowercase", chars));
    }

    if args.digits {
        let chars: Vec<char> = ('0'..='9')
            .filter(|c| !args.exclude.contains(c))
            .collect();
        char_groups.push(("digits", chars));
    }

    if args.symbols {
        let sym = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
        let chars: Vec<char> = sym
            .chars()
            .filter(|c| !args.exclude.contains(c))
            .collect();
        char_groups.push(("symbols", chars));
    }

    // Validate that at least one character type is selected
    if char_groups.is_empty() {
        eprintln!("Error: No character types selected.");
        std::process::exit(1);
    }

    if let Some((group_name, _)) = char_groups.iter().find(|(_, chars)| chars.is_empty()) {
        eprintln!(
            "Error: Cannot satisfy the selected {} requirement with the current exclusions.",
            group_name
        );
        std::process::exit(1);
    }

    if args.length < char_groups.len() {
        eprintln!(
            "Error: Password length ({}) is too short to satisfy {} selected requirements.",
            args.length,
            char_groups.len()
        );
        std::process::exit(1);
    }

    let all_chars: Vec<char> = char_groups
        .iter()
        .flat_map(|(_, chars)| chars.iter().copied())
        .collect();

    // Generate the specified number of passwords
    for _ in 0..args.count {
        let mut password_chars: Vec<char> = char_groups
            .iter()
            .map(|(_, chars)| {
                let idx = rng.gen_range(0..chars.len());
                chars[idx]
            })
            .collect();

        while password_chars.len() < args.length {
            let idx = rng.gen_range(0..all_chars.len());
            password_chars.push(all_chars[idx]);
        }

        password_chars.shuffle(&mut rng);
        let password: String = password_chars.into_iter().collect();

        println!("{}", password);
    }

    Ok(())
}
