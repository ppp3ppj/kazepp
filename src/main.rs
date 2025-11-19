// Ultra Simple Typing Practice
use rand::prelude::IndexedRandom;
// Ultra Simple Typing Practice - Fixed Row Positioning
use convert_case::{Case, Casing};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, cursor,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
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

fn render_full_screen(app: &App) -> io::Result<()> {
    let mut buffer = String::new();

    if !app.got_name {
        buffer.push_str("\r\n  Typing Practice\r\n\r\n");
        buffer.push_str("  What is your name?\r\n");
        buffer.push_str(&format!("  > {}", app.username));
        buffer.push_str("\r\n\r\n");
        buffer.push_str("  Ctrl+Q - Quit");
    } else if !app.got_mode {
        buffer.push_str(&format!("\r\n  Hello, {}!\r\n\r\n", app.username));
        buffer.push_str("  Select difficulty:\r\n\r\n");
        buffer.push_str("  1. Normal (with hint)\r\n");
        buffer.push_str("  2. Hard (no hint - lose resets score!)\r\n\r\n");
        buffer.push_str("  Press 1 or 2 to select\r\n\r\n");
        buffer.push_str("  Ctrl+S - Restart | Ctrl+Q - Quit");
    } else {
        buffer.push_str("\r\n\r\n");
        buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
        buffer.push_str(&format!("  Words:  {}\r\n\r\n", app.source));
        buffer.push_str(&format!("  Task:   Convert to {}\r\n\r\n", app.style_name));
        buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");

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

    execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    print!("{}", buffer);
    io::stdout().flush()?;
    Ok(())
}

fn update_input_line(app: &App) -> io::Result<()> {
    let input_text = if !app.got_name {
        format!("  > {}", app.username)
    } else if app.got_mode {
        format!("  Answer: {}", app.input)
    } else {
        return Ok(());
    };

    // Calculate row position - FIXED!
    let row = if !app.got_name {
        4  // Row 0-3: title + "What is your name?", Row 4: input line
    } else if app.got_mode {
        // Row 0: blank
        // Row 1: blank
        // Row 2: ━━━━
        // Row 3: blank
        // Row 4: Words:
        // Row 5: blank
        // Row 6: Task:
        // Row 7: blank
        // Row 8: ━━━━
        // Row 9: blank
        // Row 10: Hint: (Normal mode only)
        // Row 11: blank (Normal mode only)
        // Row 12: Answer: (Normal mode) OR Row 10: Answer: (Hard mode)

        if matches!(app.mode, Some(GameMode::Normal)) {
            12  // Normal mode - with hint - Answer is at row 12
        } else {
            10  // Hard mode - no hint - Answer is at row 10
        }
    } else {
        return Ok(());
    };

    // Move to input line, clear it, write new content
    execute!(
        io::stdout(),
        cursor::MoveTo(0, row),
        Clear(ClearType::CurrentLine)
    )?;
    print!("{}", input_text);
    io::stdout().flush()?;
    Ok(())
}

fn run(app: &mut App) -> Result<(), io::Error> {
    let mut needs_full_redraw = true;

    loop {
        if needs_full_redraw {
            render_full_screen(app)?;
            needs_full_redraw = false;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => {
                        if app.got_mode {
                            app.reset_score();
                            app.new_challenge();

                            execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
                            print!("\r\n\r\n  Score reset to 0!\r\n\r\n  Press any key to continue...");
                            io::stdout().flush()?;

                            event::read()?;
                            needs_full_redraw = true;
                        }
                    }
                    KeyCode::Char('s') => {
                        if app.got_name {
                            app.restart();
                            needs_full_redraw = true;
                        }
                    }
                    _ => {}
                }
                continue;
            }

            match key.code {
                KeyCode::Esc => break,
                KeyCode::Char(c) => {
                    if !app.got_name {
                        app.username.push(c);
                        update_input_line(app)?;
                    } else if !app.got_mode {
                        match c {
                            '1' => {
                                app.mode = Some(GameMode::Normal);
                                app.got_mode = true;
                                app.new_challenge();
                                needs_full_redraw = true;
                            }
                            '2' => {
                                app.mode = Some(GameMode::Hard);
                                app.got_mode = true;
                                app.new_challenge();
                                needs_full_redraw = true;
                            }
                            _ => {}
                        }
                    } else {
                        app.input.push(c);
                        update_input_line(app)?;
                    }
                }
                KeyCode::Backspace => {
                    if !app.got_name {
                        app.username.pop();
                        update_input_line(app)?;
                    } else if app.got_mode {
                        app.input.pop();
                        update_input_line(app)?;
                    }
                }
                KeyCode::Enter => {
                    if !app.got_name {
                        if !app.username.is_empty() {
                            app.got_name = true;
                            needs_full_redraw = true;
                        }
                    } else if app.got_mode {
                        let correct = app.check();

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

                        execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
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
                        needs_full_redraw = true;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
