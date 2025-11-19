
# Kazepp - Typing Practice Naming Conventions

A simple terminal-based typing practice application for learning programming naming conventions.

## Features

- **Two Difficulty Modes**
  - Normal: Practice with hints
  - Hard: No hints - wrong answer resets score to 0

- **5 Naming Styles**
  - camel case (camelCase)
  - snake case (snake_case)
  - pascal case (PascalCase)
  - kebab case (kebab-case)
  - upper snake case (UPPER_SNAKE_CASE)

- **180+ Programming Words**
  - Database terms, web development, actions, config, and more

- **Simple Clean UI**
  - No boxes, just plain text
  - Smooth rendering (optimized for Windows)
  - Real-time score tracking

## Installation

### Requirements
- Rust 1.70+ ([install from rustup.rs](https://rustup.rs))

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --release
```

## How to Play

1. **Enter your name**
2. **Choose difficulty** (1 or 2)
3. **Type the converted format**
4. **Press Enter** to check
5. **Keep practicing** to increase your score!

## Controls

| Key | Action |
|-----|--------|
| Type normally | Enter text |
| Backspace | Delete character |
| Enter | Submit answer / Continue |
| Ctrl+R | Reset score to 0 |
| Ctrl+S | Restart (back to name entry) |
| Ctrl+Q | Quit application |

## Example
```
Words:  user name email
Task:   Convert to camel case
Hint:   userNameEmail          (Normal mode only)

Answer: userNameEmail_

Score: 5
```

## Dependencies

- **crossterm** 0.29 - Terminal control
- **rand** 0.9.2 - Random generation
- **convert_case** 0.9 - Case conversion

## Makefile Commands
```bash
make run        # Run in debug mode
make release    # Build and run optimized
make build      # Build debug
make build-rel  # Build release
make build-rel-win # Build release windows
make clean      # Clean artifacts
make install    # Install system-wide
make fmt        # Format code
make clippy     # Run linter
```

## Tips

- **Normal mode**: Great for learning - see the answer before typing
- **Hard mode**: Challenge yourself - one mistake = lose all points!
- Use **Ctrl+R** to reset your score and try again
- The app is optimized for smooth rendering on Windows terminals
