# Rust CLI Password Generator - Architecture

## High-Level Architecture Diagram

```mermaid
graph TD
    subgraph "Command Line Interface Layer"
        CLI[CLI Entry Point\nmain.rs]
        Args[Argument Parsing\nclap/structopt]
        Config[Configuration Parser\nconfig.rs]
    end

    subgraph "Core Logic Layer"
        PasswordGen[Password Generator\npassword_generator.rs]
        RNG[Random Number Generator\nrand.rs]
        CharSets[Character Set Builder\ncharsets.rs]
    end

    subgraph "Utilities Layer"
        Validation[Input Validation\nvalidator.rs]
        PasswordPolicy[Password Policy Checker\npolicy.rs]
        Entropy[Entropy Calculator\nentropy.rs]
    end

    subgraph "Output Layer"
        Display[Password Display\nformatter.rs]
        Security[Secure Output\nsecure_write.rs]
    end

    CLI --> Args
    CLI --> Config
    Args --> PasswordGen
    Config --> PasswordGen
    PasswordGen --> RNG
    PasswordGen --> CharSets
    PasswordGen --> Validation
    Validation --> PasswordGen
    PasswordGen --> Entropy
    PasswordGen --> Display
    Display --> Security
```

## Detailed Module Architecture

```mermaid
graph TB
    subgraph "Entry Point"
        main[main.rs<br/>CLI Entry Point]
    end

    subgraph "Configuration"
        config[config.rs<br/>Configuration Management]
        config2[settings.rs<br/>Default Settings]
        config3[config.toml<br/>User Config File]
    end

    subgraph "Argument Parser"
        clap[clap.rs<br/>Command Line Arguments]
        subcmd[Subcommands<br/>generate/help/version]
    end

    subgraph "Core Components"
        password_gen[password_generator.rs<br/>Main Generation Logic]
        rand[rand.rs<br/>CSPRNG Integration]
        charset[charset.rs<br/>Character Set Management]
    end

    subgraph "Character Types"
        lower[lowercase_chars.rs<br/>a-z]
        upper[uppercase_chars.rs<br/>A-Z]
        digits[digits_chars.rs<br/>0-9]
        special[special_chars.rs<br/>!@#$%^&*]
    end

    subgraph "Security & Validation"
        entropy[entropy.rs<br/>Strength Analysis]
        policy[password_policy.rs<br/>Policy Enforcement]
        validate[input_validation.rs<br/>Input Sanitization]
    end

    subgraph "Output"
        formatter[formatter.rs<br/>Output Formatting]
        secure[secure_io.rs<br/>Secure Output Writing]
    end

    main --> clap
    main --> config
    clap --> subcmd
    config --> config2
    config --> config3
    subcmd --> password_gen
    password_gen --> rand
    password_gen --> charset
    password_gen --> lower
    password_gen --> upper
    password_gen --> digits
    password_gen --> special
    password_gen --> entropy
    password_gen --> policy
    password_gen --> validate
    password_gen --> formatter
    password_gen --> secure
```

## Data Flow Diagram

```mermaid
sequenceDiagram
    participant User as User
    participant CLI as CLI Parser
    participant Config as Config
    participant Validate as Validator
    participant Charset as CharSet Manager
    participant Rand as CSPRNG
    participant Generator as Password Generator
    participant Formatter as Formatter
    participant Secure as Secure Output

    User->>CLI: --length=12 --lower --upper --digits --special
    User->>Config: --config config.toml
    CLI->>Validate: Validate arguments
    Validate-->>CLI: ✓ Valid or ✗ Error message
    Config->>Charset: Get character sets based on flags
    Charset-->>Generator: [lower, upper, digits, special]
    Rand->>Rand: Generate secure random bytes
    Rand-->>Generator: Random index values
    Generator->>Generator: Build password string
    Generator->>Formatter: Format output
    Formatter->>Secure: Write to stdout
    Secure-->>User: Password
```

## Component Dependencies

```mermaid
mindmap
  root((Password Generator))
    CLI Layer
      main.rs
        Entry point
        Argument parsing
        Subcommands
    Configuration
      config.rs
        TOML parsing
        Default values
        Environment variables
      config.toml
        Length
        Character sets
        Output format
    Core
      password_generator.rs
        Generation loop
        Set management
      rand.rs
        /dev/urandom
        rand::random
      charset.rs
        Character pools
        Set management
    Security
      entropy.rs
        Shannon entropy
        Bit strength
      password_policy.rs
        Min/max length
        Character requirements
    Output
      formatter.rs
        One-liner
        Multi-line
      secure_io.rs
        /dev/stdout
        Buffer flushing
```

## File Structure

```mermaid
mindmap
  type((mini-project-password-generator/))
    Cargo.toml
      Dependencies
      Metadata
    src
      main.rs
      password_generator.rs
      config.rs
      rand.rs
      charset.rs
      entropy.rs
      policy.rs
      validation.rs
      formatter.rs
      secure_io.rs
    tests
      integration_tests.rs
      generation_tests.rs
    docs
      architecture.md
      API.md
    config.toml
      Sample configuration
    README.md
    Cargo.lock
```

## Example Command Flow

```mermaid
flowchart LR
    A[User runs command] --> B{Parse arguments}
    B -->|Success| C[Load config]
    B -->|Error| D[Show error]
    D --> E[Exit]
    C --> F{Validate input}
    F -->|Pass| G[Build char sets]
    F -->|Fail| D
    G --> H[Generate password]
    H --> I[Check entropy]
    I --> J[Format output]
    J --> K[Display password]
```

## Key Technologies Used

| Component | Technology | Purpose |
|-----------|-----------|---------|
| CLI Parsing | `clap` or `structopt` | Command-line argument handling |
| Random Generation | `rand` + `/dev/urandom` | Cryptographically secure RNG |
| Configuration | `toml` | Structured config file |
| Validation | Custom modules | Input validation |
| Output | Direct stdout | Secure display |

## Security Considerations

```mermaid
graph LR
    A[User Input] --> B{Validate}
    B -->|Clean| C[Secure RNG]
    C --> D[Character Sets]
    D --> E[Generate]
    E --> F{Entropy Check}
    F -->|Insufficient| G[Warning]
    F -->|Adequate| H[Output]
    H --> I[Clear Memory]
    I --> J[Exit]
    G --> J
    B -->|Malicious| J
```