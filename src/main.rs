// Ultra Simple Typing Practice
use rand::prelude::IndexedRandom;
use convert_case::{Case, Casing};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    cursor,
};
use rand::{seq::SliceRandom, Rng};
use std::io::{self, Write};

const WORDS: &[&str] = &[
    // Basic
    "user", "name", "email", "password", "address", "phone", "message", "text",
    "data", "file", "folder", "path", "link", "url", "image", "video",

    // Database
    "database", "table", "column", "row", "query", "record", "index", "key",
    "primary", "foreign", "schema", "migration", "transaction", "connection",

    // Programming
    "function", "method", "class", "object", "variable", "constant", "array", "list",
    "string", "number", "boolean", "null", "undefined", "type", "interface", "struct",

    // Actions
    "create", "read", "update", "delete", "insert", "select", "remove", "add",
    "get", "set", "fetch", "send", "post", "put", "patch", "save",
    "load", "download", "upload", "export", "import", "parse", "format", "convert",

    // Web
    "server", "client", "request", "response", "api", "endpoint", "route", "handler",
    "controller", "model", "view", "service", "repository", "middleware", "filter",

    // Common modifiers
    "first", "last", "next", "previous", "current", "total", "count", "sum",
    "max", "min", "average", "new", "old", "active", "inactive", "enabled",
    "disabled", "visible", "hidden", "public", "private", "protected", "static",

    // Status
    "success", "error", "warning", "info", "pending", "complete", "failed", "valid",
    "invalid", "required", "optional", "default", "custom", "standard", "temp",

    // Common words
    "item", "list", "array", "map", "set", "queue", "stack", "tree", "graph",
    "node", "edge", "parent", "child", "root", "leaf", "level", "depth",

    // Time related
    "time", "date", "timestamp", "created", "updated", "deleted", "start", "end",
    "duration", "timeout", "interval", "schedule", "delay", "expired",

    // Auth & Security
    "auth", "token", "session", "cookie", "login", "logout", "register", "verify",
    "encrypt", "decrypt", "hash", "salt", "secure", "permission", "role", "access",

    // Common operations
    "sort", "filter", "search", "find", "match", "compare", "merge", "split",
    "join", "concat", "append", "prepend", "replace", "remove", "clear", "reset",

    // Config & Settings
    "config", "setting", "option", "preference", "parameter", "argument", "value",
    "flag", "toggle", "switch", "mode", "state", "status", "level", "priority",
];

const STYLES: &[(&str, Case)] = &[
    ("camel case", Case::Camel),
    ("snake case", Case::Snake),
    ("pascal case", Case::Pascal),
    ("kebab case", Case::Kebab),
    ("upper snake case", Case::UpperSnake),
];

enum GameMode {
    Normal,
    Hard,
}

struct App {
    username: String,
    source: String,
    hint: String,
    style_name: String,
    input: String,
    score: usize,
    got_name: bool,
    got_mode: bool,
    mode: Option<GameMode>,
}

impl App {
    fn new() -> Self {
        Self {
            username: String::new(),
            source: String::new(),
            hint: String::new(),
            style_name: String::new(),
            input: String::new(),
            score: 0,
            got_name: false,
            got_mode: false,
            mode: None,
        }
    }

    fn new_challenge(&mut self) {
        let mut rng = rand::thread_rng();
        let word_count = rng.random_range(2..=4);
        let mut words = Vec::new();
        for _ in 0..word_count {
            if let Some(word) = WORDS.choose(&mut rng) {
                words.push(*word);
            }
        }

        let (style_name, case) = STYLES.choose(&mut rng).unwrap();
        self.source = words.join(" ");
        self.hint = self.source.to_case(*case);
        self.style_name = style_name.to_string();
        self.input.clear();
    }

    fn check(&mut self) -> bool {
        self.input.trim() == self.hint
    }

    fn reset_score(&mut self) {
        self.score = 0;
    }

    fn restart(&mut self) {
        self.score = 0;
        self.got_name = false;
        self.got_mode = false;
        self.mode = None;
        self.username.clear();
        self.input.clear();
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    let mut app = App::new();
    let _ = run(&mut app);

    execute!(stdout, cursor::Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn run(app: &mut App) -> Result<(), io::Error> {
    loop {
        // Build entire screen in buffer first
        let mut buffer = String::new();

        if !app.got_name {
            // Page 1: Name entry
            buffer.push_str("\r\n  Typing Practice\r\n\r\n");
            buffer.push_str("  What is your name?\r\n");
            buffer.push_str(&format!("  > {}", app.username));
            buffer.push_str("\r\n\r\n");
            buffer.push_str("  Ctrl+Q - Quit");
        } else if !app.got_mode {
            // Page 2: Mode selection
            buffer.push_str(&format!("\r\n  Hello, {}!\r\n\r\n", app.username));
            buffer.push_str("  Select difficulty:\r\n\r\n");
            buffer.push_str("  1. Normal (with hint)\r\n");
            buffer.push_str("  2. Hard (no hint - lose resets score!)\r\n\r\n");
            buffer.push_str("  Press 1 or 2 to select\r\n\r\n");
            buffer.push_str("  Ctrl+S - Restart | Ctrl+Q - Quit");
        } else {
            // Page 3: Practice
            buffer.push_str("\r\n\r\n");
            buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
            buffer.push_str(&format!("  Words:  {}\r\n\r\n", app.source));
            buffer.push_str(&format!("  Task:   Convert to {}\r\n\r\n", app.style_name));
            buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");

            // Show hint only in Normal mode
            if matches!(app.mode, Some(GameMode::Normal)) {
                buffer.push_str(&format!("  Hint:   {}\r\n\r\n", app.hint));
            }

            buffer.push_str(&format!("  Answer: {}", app.input));
            buffer.push_str("\r\n\r\n");
            buffer.push_str(&format!("  Score: {}\r\n", app.score));
            buffer.push_str("\r\n");
            buffer.push_str("  Ctrl+R - Reset Score | Ctrl+S - Restart | Ctrl+Q - Quit");
            buffer.push_str("\r\n");
        }

        // Clear and draw everything at once - SMOOTH!
        clear_screen()?;
        print!("{}", buffer);
        io::stdout().flush()?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            // Handle Ctrl shortcuts
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('q') => {
                        // Ctrl+Q: Quit
                        break;
                    }
                    KeyCode::Char('r') => {
                        // Ctrl+R: Reset score
                        if app.got_mode {
                            app.reset_score();
                            app.new_challenge();

                            let mut buffer = String::new();
                            buffer.push_str("\r\n\r\n");
                            buffer.push_str("  Score reset to 0!\r\n\r\n");
                            buffer.push_str("  Press any key to continue...");

                            clear_screen()?;
                            print!("{}", buffer);
                            io::stdout().flush()?;

                            event::read()?;
                        }
                    }
                    KeyCode::Char('s') => {
                        // Ctrl+S: Full restart
                        if app.got_name {
                            app.restart();
                        }
                    }
                    _ => {}
                }
                continue;
            }

            // Normal key handling
            match key.code {
                KeyCode::Esc => break,  // Keep Esc as alternative quit
                KeyCode::Char(c) => {
                    if !app.got_name {
                        app.username.push(c);
                    } else if !app.got_mode {
                        match c {
                            '1' => {
                                app.mode = Some(GameMode::Normal);
                                app.got_mode = true;
                                app.new_challenge();
                            }
                            '2' => {
                                app.mode = Some(GameMode::Hard);
                                app.got_mode = true;
                                app.new_challenge();
                            }
                            _ => {}
                        }
                    } else {
                        app.input.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if !app.got_name {
                        app.username.pop();
                    } else if app.got_mode {
                        app.input.pop();
                    }
                }
                KeyCode::Enter => {
                    if !app.got_name {
                        if !app.username.is_empty() {
                            app.got_name = true;
                        }
                    } else if app.got_mode {
                        let correct = app.check();

                        // Build result screen in buffer
                        let mut buffer = String::new();

                        if correct {
                            app.score += 1;
                            buffer.push_str("\r\n\r\n");
                            buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                            buffer.push_str("  ✓ CORRECT!\r\n\r\n");
                            buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                            buffer.push_str(&format!("  Style:      {}\r\n", app.style_name));
                            buffer.push_str(&format!("  Answer:     {}\r\n", app.hint));
                            buffer.push_str(&format!("  You typed:  {}\r\n\r\n", app.input));
                            buffer.push_str(&format!("  Score: {}\r\n\r\n", app.score));
                        } else {
                            if matches!(app.mode, Some(GameMode::Hard)) {
                                buffer.push_str("\r\n\r\n");
                                buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                                buffer.push_str("  ✗ YOU LOSE!\r\n\r\n");
                                buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                                buffer.push_str(&format!("  Style:      {}\r\n", app.style_name));
                                buffer.push_str(&format!("  Answer:     {}\r\n", app.hint));
                                buffer.push_str(&format!("  You typed:  {}\r\n\r\n", app.input));
                                buffer.push_str("  Score reset to 0\r\n\r\n");
                                app.score = 0;
                            } else {
                                buffer.push_str("\r\n\r\n");
                                buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                                buffer.push_str("  ✗ Wrong\r\n\r\n");
                                buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                                buffer.push_str(&format!("  Style:      {}\r\n", app.style_name));
                                buffer.push_str(&format!("  Answer:     {}\r\n", app.hint));
                                buffer.push_str(&format!("  You typed:  {}\r\n\r\n", app.input));
                                buffer.push_str(&format!("  Score: {}\r\n\r\n", app.score));
                            }
                        }

                        buffer.push_str("  Press Enter to continue...");

                        clear_screen()?;
                        print!("{}", buffer);
                        io::stdout().flush()?;

                        loop {
                            if let Event::Key(key) = event::read()? {
                                if key.kind == KeyEventKind::Press && key.code == KeyCode::Enter {
                                    break;
                                }
                            }
                        }

                        app.new_challenge();
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn clear_screen() -> Result<(), io::Error> {
    execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))
}
