# 🔐 Password Generator

A **secure command-line password generator** written in Rust, designed to generate cryptographically strong passwords for your various needs.

## ✨ Features

- 🛡️ **Cryptographically Secure**: Uses `OsRng` for secure random number generation
- 🔧 **Fully Configurable**: Customize password length and character sets
- 🎯 **Selective Exclusions**: Exclude specific character types or characters
- 💻 **Cross-Platform**: Works on Linux, macOS, and Windows
- 📦 **Zero External Binaries**: Single binary, no runtime dependencies
- 🧪 **Tested**: Includes comprehensive unit and integration tests

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/youruser/password-generator.git
cd password-generator

# Build the release binary
cargo build --release

# Run the generator
./target/release/password-generator
```

### Installation on Linux/macOS

```bash
# Install with cargo
cargo install --path . --root /usr/local

# Or add to PATH directly
export PATH="$PWD/target/release:$PATH"
```

## 🚀 Usage

### Basic Usage

```bash
# Generate a 12-character password (default)
./target/release/password-generator

# Generate a longer password
./target/release/password-generator --length 20

# Generate multiple passwords
./target/release/password-generator --count 5
```

### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--length` | `-l` | Password length | 12 |
| `--upper` | `-u` | Include uppercase letters | true |
| `--lower` | `-L` | Include lowercase letters | true |
| `--digits` | `-d` | Include numbers | true |
| `--symbols` | `-s` | Include symbols | true |
| `--exclude` | `-e` | Exclude character types | none |
| `--count` | `-c` | Number of passwords to generate | 1 |

### Examples

```bash
# Generate a password with only lowercase and numbers
./target/release/password-generator --lower --digits --length 16

# Generate a password without special characters
./target/release/password-generator --exclude '!' --exclude '"' --exclude '#'

# Generate 10 passwords of length 16
./target/release/password-generator --length 16 --count 10

# Generate secure passwords for email (no symbols, uppercase, digits)
./target/release/password-generator --length 14 --exclude '"~`{}|!@#$%^&*()_+-=[]\;:\'",\.<>?/'
```

### Using Environment Variables

The generator can use environment variables for default settings:

```bash
# Set default length
export PASSWORD_LENGTH=20

# Set default character types
export PASSWORD_UPPER=true
export PASSWORD_DIGITS=true
export PASSWORD_SYMBOLS=true
```

## 🧪 Running Tests

```bash
# Run all tests
cargo test

# Run tests with coverage
cargo tarpaulin -o html

# Run specific test
cargo test --test integration
```

## 📋 Project Structure

```
password-generator/
├── src/
│   └── main.rs          # Main application logic
├── Cargo.toml           # Dependencies and build configuration
├── .gitignore           # Git ignore rules
├── README.md            # This file
├── .env                 # Environment variables (optional)
└── tests/
    └── integration.rs   # Integration tests
```

## 🔒 Security Considerations

- Uses `OsRng` for cryptographically secure random number generation
- No hardcoded secrets or passwords in the codebase
- Dependencies are audited for vulnerabilities
- Single binary reduces attack surface

## 📝 License

This project is licensed under the MIT License - see the `LICENSE` file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 🙏 Acknowledgments

- [Rust](https://www.rust-lang.org/) for building a safe and secure language
- [rand](https://crates.io/crates/rand) for random number generation
- [clap](https://crates.io/crates/clap) for command-line parsing

## 📊 Benchmarks

<<<<<<< HEAD
| Length | Time per Password |
|--------|-------------------|
| 12     | ~0.5ms            |
| 20     | ~0.8ms            |
| 32     | ~1.2ms            |

*Tests conducted on Intel i7-10700K*

---

**Made with ❤️ by Duskthoth**
=======
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
>>>>>>> fe5ad58010641517f02bde72266e7191dfd94fb6
