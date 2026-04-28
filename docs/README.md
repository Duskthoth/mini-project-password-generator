# Password Generator Documentation

## What is Password Generator?

Password Generator is a Rust command-line utility that generates cryptographically secure random passwords for your daily use.

## Installation

1. **Clone or navigate to the project directory:**

```bash
cd /var/home/dusk/Documents/mini-project-password-generator
```

2. **Build the project:**

```bash
cargo build --release
```

3. **Run the executable:**

```bash
./target/release/password-generator
```

## Quick Start

### Basic Usage

```bash
# Generate one password (default: 12 characters, all types)
password-generator

# Generate 5 passwords
password-generator --count 5

# Generate longer passwords
password-generator --length 20
```

### Customization Options

```bash
# Generate passwords without digits
password-generator --digits false

# Generate passwords with only uppercase and symbols
password-generator --digits false --lower false

# Generate passwords excluding special characters
password-generator --symbols false

# Generate multiple passwords without certain types
password-generator --length 16 --count 10 --digits false --symbols false
```

### Advanced Usage

```bash
# Generate 3 passwords of length 20 without symbols
password-generator --length 20 --count 3 --symbols false

# Generate passwords with custom length and count
password-generator -l 18 -c 5
```

## Features

- **Cryptographically Secure**: Uses ChaCha8Rng for secure random number generation
- **Flexible Configuration**: Easily customize password composition
- **Multiple Passwords**: Generate multiple passwords at once
- **User-Friendly**: Clear help messages and error handling
- **Optimized Performance**: Fast password generation with minimal overhead

## Security

- Uses `ChaCha8Rng` from the `rand_chacha` crate
- Seeded with operating system entropy via `OsRng`
- All selected character types are equally likely to be chosen
- No predictable patterns or biases

## Architecture Overview

The application consists of four main components:

1. **Arguments Parser**: Uses `clap` for type-safe CLI argument parsing
2. **Random Number Generator**: Uses `rand_chacha::ChaCha8Rng` for secure randomness
3. **Character Set Builder**: Filters character types based on user preferences
4. **Password Generator**: Generates passwords by randomly selecting from the filtered character set

## Project Structure

```
password-generator/
├── Cargo.toml              # Dependencies and configuration
├── docs/
│   ├── architecture.md     # Technical architecture documentation
│   └── documentation.md    # User-facing documentation (this file)
├── src/
│   └── main.rs             # Main application source code
└── target/
    └── release/
        └── password-generator/  # Compiled binary
```

## Configuration

### Dependencies (Cargo.toml)

```toml
[dependencies]
clap = { version = "4.4.11", features = ["derive"] }
rand = "0.8.5"
rand_chacha = "0.3.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Core Components

- **Arguments Parser**: `clap` provides type-safe argument definitions
- **RNG**: `rand_chacha::ChaCha8Rng` for cryptographically secure generation
- **Character Set**: Built from uppercase, lowercase, digits, and symbols
- **Password Generator**: Random selection with proper filtering

## Command-Line Reference

```
USAGE:
    password-generator [OPTIONS]

OPTIONS:
    -h, --help         Print help information
    -V, --version      Print version information
    -l, --length <N>   Password length (default: 12)
    -c, --count <N>    Number of passwords to generate (default: 1)
    --upper            Include uppercase letters (default: true)
    --lower            Include lowercase letters (default: true)
    --digits           Include digits (default: true)
    --symbols          Include symbols (default: true)
    --exclude <TYPES>  Exclude specified character types
```

## Examples

### Generate Passwords

```bash
# Single password (default settings)
password-generator

# Multiple passwords
password-generator --count 5

# Custom length
password-generator --length 20

# Exclude certain types
password-generator --digits false --symbols false
```

### Advanced Scenarios

```bash
# Generate for email accounts (alphanumeric)
password-generator --upper true --lower true --digits true --symbols false --count 10

# Generate for database accounts (complex)
password-generator --upper true --lower true --digits true --symbols true --length 24

# Generate for API keys (hexadecimal)
password-generator --upper false --lower false --digits true --symbols false --length 64
```

## Error Handling

- **Empty Character Set**: Exits with error if no character types are selected
- **Invalid Arguments**: Provides helpful error messages and suggestions
- **Default Behavior**: Gracefully defaults to all character types

## Performance

- **Fast Generation**: Uses efficient `ChaCha8Rng`
- **Minimal Overhead**: Simple character set filtering
- **Optimized Build**: Compiled with `--release` flag

## Future Improvements

Potential enhancements:

1. Custom character set input option
2. Password pattern validation
3. Progress indicator for large counts
4. Export passwords to files (JSON, CSV)
5. Password strength meter
6. Support for locale-specific character sets

## Contributing

This project is maintained by [Duskthoth]. Feel free to submit issues or pull requests.

## License

This project is licensed under the MIT License.

## Support

For issues or questions, please open an issue on the GitHub repository.
