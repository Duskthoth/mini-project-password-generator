use clap::Parser;
use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

type CharGroup = (&'static str, Vec<char>);

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

    let char_groups = build_char_groups(&args);
    if let Err(error) = validate_char_groups(&char_groups, args.length) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
    let all_chars = build_all_chars(&char_groups);

    // Generate the specified number of passwords
    for _ in 0..args.count {
        let password = generate_password(args.length, &char_groups, &all_chars, &mut rng);
        println!("{}", password);
    }

    Ok(())
}

fn build_char_groups(args: &Args) -> Vec<CharGroup> {
    let mut char_groups: Vec<CharGroup> = Vec::new();

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

    char_groups
}

fn validate_char_groups(char_groups: &[CharGroup], length: usize) -> Result<(), String> {
    if char_groups.is_empty() {
        return Err("No character types selected.".to_string());
    }

    if let Some((group_name, _)) = char_groups.iter().find(|(_, chars)| chars.is_empty()) {
        return Err(format!(
            "Cannot satisfy the selected {} requirement with the current exclusions.",
            group_name
        ));
    }

    if length < char_groups.len() {
        return Err(format!(
            "Password length ({}) is too short to satisfy {} selected requirements.",
            length,
            char_groups.len()
        ));
    }

    Ok(())
}

fn build_all_chars(char_groups: &[CharGroup]) -> Vec<char> {
    char_groups
        .iter()
        .flat_map(|(_, chars)| chars.iter().copied())
        .collect()
}

fn generate_password<R: Rng + ?Sized>(
    length: usize,
    char_groups: &[CharGroup],
    all_chars: &[char],
    rng: &mut R,
) -> String {
    let mut password_chars: Vec<char> = char_groups
        .iter()
        .map(|(_, chars)| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx]
        })
        .collect();

    while password_chars.len() < length {
        let idx = rng.gen_range(0..all_chars.len());
        password_chars.push(all_chars[idx]);
    }

    password_chars.shuffle(rng);
    password_chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_args() -> Args {
        Args {
            length: 12,
            upper: false,
            lower: false,
            digits: false,
            symbols: false,
            exclude: Vec::new(),
            count: 1,
        }
    }

    #[test]
    fn build_char_groups_applies_exclusions() {
        let mut args = default_args();
        args.lower = true;
        args.exclude = vec!['a', 'z'];

        let groups = build_char_groups(&args);
        let lowercase_group = groups
            .iter()
            .find(|(name, _)| *name == "lowercase")
            .expect("lowercase group should exist");

        assert!(!lowercase_group.1.contains(&'a'));
        assert!(!lowercase_group.1.contains(&'z'));
    }

    #[test]
    fn validate_char_groups_fails_without_selected_groups() {
        let groups: Vec<CharGroup> = Vec::new();
        let result = validate_char_groups(&groups, 8);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No character types selected.");
    }

    #[test]
    fn validate_char_groups_fails_when_length_is_insufficient() {
        let groups: Vec<CharGroup> = vec![
            ("lowercase", vec!['a', 'b']),
            ("digits", vec!['1', '2']),
            ("symbols", vec!['!']),
        ];
        let result = validate_char_groups(&groups, 2);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("too short to satisfy 3 selected requirements")
        );
    }

    #[test]
    fn validate_char_groups_fails_when_selected_group_is_empty() {
        let groups: Vec<CharGroup> = vec![("lowercase", Vec::new())];
        let result = validate_char_groups(&groups, 4);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Cannot satisfy the selected lowercase requirement")
        );
    }

    #[test]
    fn generate_password_contains_all_selected_requirements() {
        let groups: Vec<CharGroup> = vec![
            ("lowercase", vec!['a', 'b', 'c']),
            ("digits", vec!['1', '2', '3']),
            ("symbols", vec!['!', '@']),
        ];
        let all_chars = build_all_chars(&groups);
        let mut rng = ChaCha8Rng::seed_from_u64(42);

        let password = generate_password(6, &groups, &all_chars, &mut rng);

        assert_eq!(password.len(), 6);
        assert!(password.chars().any(|c| c.is_ascii_lowercase()));
        assert!(password.chars().any(|c| c.is_ascii_digit()));
        assert!(password.chars().any(|c| !c.is_ascii_alphanumeric()));
    }
}
