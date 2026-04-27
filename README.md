# Secure Password Generator

A secure command-line password generator written in Rust that generates cryptographically secure random passwords based on configurable character types.

## Features

- **Secure Random Generation**: Uses Rust's `rand` crate combined with `/dev/urandom` for cryptographically secure random number generation
- **Configurable Character Types**: Select from lowercase letters, uppercase letters, digits, and special characters
- **Custom Length**: Specify password length via command-line arguments
- **Entropy Calculation**: Built-in strength analysis to ensure password security
- **Configurable**: Supports TOML configuration files for persistent settings
- **CLI Interface**: User-friendly command-line interface with subcommands
- **Secure Output**: Proper memory handling to prevent password leakage

## Architecture

This project follows a modular architecture with the following layers:

```
┌─────────────────────────────────────────────────────────┐
│                   Command Line Interface                  │
│  - main.rs (Entry point)                                 │
│  - clap/structopt (Argument parsing)                      │
│  - config.rs (Configuration management)                   │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│                     Core Logic Layer                      │
│  - password_generator.rs (Generation logic)               │
│  - rand.rs (CSPRNG integration)                          │
│  - charset.rs (Character set management)                  │
│  - entropy.rs (Strength analysis)                         │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│                    Output & Validation Layer              │
│  - formatter.rs (Output formatting)                       │
│  - secure_io.rs (Secure output writing)                   │
│  - validator.rs (Input validation)                        │
│  - policy.rs (Password policy enforcement)                │
└─────────────────────────────────────────────────────────┘
```

## Installation

### Prerequisites

- Rust toolchain: `rustc` 1.70+
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd mini-project-password-generator

# Build the project
cargo build --release

# Run the binary
./target/release/password-generator
```

### Dependencies

This project uses the following crates:

```toml
[dependencies]
rand = "0.8"          # Random number generation
rand_chacha = "0.3"   # ChaCha20 RNG
getrandom = "0.2"     # System RNG access
clap = "4.0"          # CLI argument parsing
toml = "0.5"          # TOML configuration parsing
```

## Usage

### Basic Usage

```bash
# Generate a 12-character password with all character types
password-generator --length 12 --lower --upper --digits --special

# Generate a 16-character password (recommended for high security)
password-generator --length 16 --lower --upper --digits --special

# Generate a password with specific character types
password-generator --length 10 --lower --upper --digits
```

### Configuration File

Create a `config.toml` file in the project root:

```toml
[default]
length = 16
include_lower = true
include_upper = true
include_digits = true
include_special = true
output_format = "single"
```

Then run:

```bash
password-generator --config config.toml
```

### Subcommands

```bash
# Generate password
password-generator generate

# Show help
password-generator help

# Show version
password-generator version
```

## Examples

```bash
# Quick password (8 chars)
password-generator --length 8

# Strong password (20 chars with all types)
password-generator --length 20 --all

# Password without special characters
password-generator --length 12 --lower --upper --digits

# Custom character set (requires --chars flag)
password-generator --length 14 --chars "ABCdef123!@#"
```

## Security Considerations

- Uses CSPRNG (Cryptographically Secure Pseudo-Random Number Generator)
- Minimum entropy threshold enforced before password acceptance
- Secure memory handling prevents password leakage
- No password is logged or stored
- Output is written directly to stdout without intermediate storage

## Testing

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run with coverage
cargo test -- --nocapture --test-threads=1
```

## Configuration Options

| Option | Description | Default |
|--------|-------------|---------|
| `--length` | Password length | 16 |
| `--lower` | Include lowercase letters (a-z) | true |
| `--upper` | Include uppercase letters (A-Z) | true |
| `--digits` | Include digits (0-9) | true |
| `--special` | Include special characters (!@#$%^&*) | true |
| `--config` | Path to config.toml | None |
| `--output` | Output format (single/multi) | single |
| `--entropy-min` | Minimum entropy (bits) | 128 |

## License

MIT License - See LICENSE file for details

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## Architecture Documentation

For detailed architecture diagrams and component breakdowns, see:
- `docs/architecture.md` - Comprehensive architecture documentation
- `docs/mermaid-diagrams.mmd` - Mermaid diagrams for visualization

## Project Structure

```
mini-project-password-generator/
├── Cargo.toml           # Project dependencies and metadata
├── Cargo.lock           # Locked dependencies
├── README.md            # This file
├── docs/
│   ├── architecture.md  # Architecture documentation
│   └── mermaid-diagrams.mmd # Mermaid diagram source
├── src/
│   ├── main.rs          # CLI entry point
│   ├── password_generator.rs
│   ├── config.rs
│   ├── rand.rs
│   ├── charset.rs
│   ├── entropy.rs
│   └── ...              # Other modules
└── tests/
    └── tests.rs         # Integration tests
```
