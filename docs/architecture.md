# Password Generator - Architecture

## Overview

The Password Generator is a command-line utility written in Rust that generates cryptographically secure random passwords.

## Project Structure

```
password-generator/
├── Cargo.toml              # Project dependencies
├── docs/
│   ├── architecture.md     # This file
│   └── documentation.md    # General documentation
├── src/
│   └── main.rs             # Main source code
└── target/
    └── release/
        └── password-generator/  # Compiled binary
```

## Dependencies

### Cargo.toml

```toml
[package]
name = "password-generator"
version = "0.1.0"
edition = "2021"
authors = ["Duskthoth"]

[dependencies]
clap = { version = "4.4.11", features = ["derive"] }  # CLI parsing
rand = "0.8.5"                                        # Random generation
rand_chacha = "0.3.1"                                 # Secure RNG
serde = { version = "1.0", features = ["derive"] }    # Serialization
serde_json = "1.0"                                    # JSON support
```

## Core Components

### 1. Arguments Parser (Clap)

Uses `clap` for command-line argument parsing with:
- Type-safe argument definitions
- Automatic help generation
- Error handling and suggestions
- Default values

### 2. Random Number Generator

Uses `rand_chacha::ChaCha8Rng` for cryptographically secure random numbers.
Seeded with `rand::rngs::OsRng` for true randomness from OS.

### 3. Character Set Builder

Builds character set based on user preferences:
- Uppercase letters (A-Z)
- Lowercase letters (a-z)
- Digits (0-9)
- Special symbols

### 4. Password Generator

1. Filters character set based on user options
2. For each character position:
   - Generates random index
   - Selects character at that index
   - Appends to password string
3. Returns generated password

## Command-Line Interface

### Usage

```bash
password-generator [OPTIONS]
```

### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--length` | `-l` | Password length | 12 |
| `--upper` | - | Include uppercase letters | true |
| `--lower` | - | Include lowercase letters | true |
| `--digits` | - | Include digits | true |
| `--symbols` | - | Include symbols | true |
| `--exclude` | `-e` | Exclude character types | [] |
| `--count` | `-c` | Number of passwords | 1 |

### Examples

```bash
# Generate one password (default 12 chars)
password-generator

# Generate 5 passwords of length 16
password-generator --length 16 --count 5

# Generate without digits
password-generator --digits false
```

## Security Considerations

- **Cryptographically Secure RNG**: Uses ChaCha8Rng
- **Seeding**: Randomly seeded with OS entropy via `OsRng`
- **Character Distribution**: All types equally likely

## Performance

- **Fast Generation**: Efficient ChaCha8Rng
- **Minimal Overhead**: Simple filtering
- **Optimized Build**: Compiled with `--release`

## Error Handling

- Empty character set exits with error code 1
- Invalid arguments show helpful messages
- Defaults to all types unless disabled

## Future Improvements

1. Custom character set input
2. Pattern validation
3. Progress indicator
4. Export to files (JSON, CSV)
5. Password strength meter
6. Locale-specific character sets
