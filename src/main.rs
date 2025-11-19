// Ultra Simple Typing Practice
use rand::prelude::IndexedRandom;
use convert_case::{Case, Casing};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, cursor,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    style::Print,
};
use rand::{seq::SliceRandom, rngs::ThreadRng, Rng};
use std::io;
use std::fmt::Write as FmtWrite;

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
    rng: ThreadRng,
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
            rng: rand::thread_rng(),
        }
    }

    fn new_challenge(&mut self) {
        let word_count = self.rng.random_range(2..=4);
        let mut words = Vec::with_capacity(word_count);

        for _ in 0..word_count {
            if let Some(word) = WORDS.choose(&mut self.rng) {
                words.push(*word);
            }
        }

        let (style_name, case) = STYLES.choose(&mut self.rng).unwrap();
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
    let mut buffer = String::with_capacity(512);

    if !app.got_name {
        buffer.push_str("\r\n Kezepp: Naming convention typing practice\r\n\r\n");
        buffer.push_str("  What is your name?\r\n");
        let _ = write!(buffer, "  > {}", app.username);
        buffer.push_str("\r\n\r\n");
        buffer.push_str("  Ctrl+Q - Quit");
    } else if !app.got_mode {
        let _ = write!(buffer, "\r\n  Hello, {}!\r\n\r\n", app.username);
        buffer.push_str("  Select difficulty:\r\n\r\n");
        buffer.push_str("  1. Normal (with hint)\r\n");
        buffer.push_str("  2. Hard (no hint - lose resets score!)\r\n\r\n");
        buffer.push_str("  Press 1 or 2 to select\r\n\r\n");
        buffer.push_str("  Ctrl+S - Restart | Ctrl+Q - Quit");
    } else {
        buffer.push_str("\r\n\r\n");
        buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
        let _ = write!(buffer, "  Words:  {}\r\n\r\n", app.source);
        let _ = write!(buffer, "  Task:   Convert to {}\r\n\r\n", app.style_name);
        buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");

        if matches!(app.mode, Some(GameMode::Normal)) {
            let _ = write!(buffer, "  Hint:   {}\r\n\r\n", app.hint);
        }

        let _ = write!(buffer, "  Answer: {}", app.input);
        buffer.push_str("\r\n\r\n");
        let _ = write!(buffer, "  Score: {}\r\n", app.score);
        buffer.push_str("\r\n");
        buffer.push_str("  Ctrl+R - Reset Score | Ctrl+S - Restart | Ctrl+Q - Quit");
        buffer.push_str("\r\n");
    }

    execute!(
        io::stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        Print(&buffer)
    )?;
    Ok(())
}

fn update_input_line(app: &App) -> io::Result<()> {
    let mut input_text = String::with_capacity(128);

    if !app.got_name {
        let _ = write!(input_text, "  > {}", app.username);
    } else if app.got_mode {
        let _ = write!(input_text, "  Answer: {}", app.input);
    } else {
        return Ok(());
    }

    let row = if !app.got_name {
        4
    } else if app.got_mode {
        if matches!(app.mode, Some(GameMode::Normal)) { 12 } else { 10 }
    } else {
        return Ok(());
    };

    execute!(
        io::stdout(),
        cursor::MoveTo(0, row),
        Clear(ClearType::CurrentLine),
        Print(&input_text)
    )?;
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

                            let mut buffer = String::with_capacity(128);
                            buffer.push_str("\r\n\r\n  Score reset to 0!\r\n\r\n  Press any key to continue...");

                            execute!(
                                io::stdout(),
                                Clear(ClearType::All),
                                cursor::MoveTo(0, 0),
                                Print(&buffer)
                            )?;

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

                        let mut buffer = String::with_capacity(512);

                        if correct {
                            app.score += 1;
                            buffer.push_str("\r\n\r\n");
                            buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                            buffer.push_str("  ✓ CORRECT!\r\n\r\n");
                            buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                            let _ = write!(buffer, "  Style:      {}\r\n", app.style_name);
                            let _ = write!(buffer, "  Answer:     {}\r\n", app.hint);
                            let _ = write!(buffer, "  You typed:  {}\r\n\r\n", app.input);
                            let _ = write!(buffer, "  Score: {}\r\n\r\n", app.score);
                        } else {
                            if matches!(app.mode, Some(GameMode::Hard)) {
                                buffer.push_str("\r\n\r\n");
                                buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                                buffer.push_str("  ✗ YOU LOSE!\r\n\r\n");
                                buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                                let _ = write!(buffer, "  Style:      {}\r\n", app.style_name);
                                let _ = write!(buffer, "  Answer:     {}\r\n", app.hint);
                                let _ = write!(buffer, "  You typed:  {}\r\n\r\n", app.input);
                                buffer.push_str("  Score reset to 0\r\n\r\n");
                                app.score = 0;
                            } else {
                                buffer.push_str("\r\n\r\n");
                                buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                                buffer.push_str("  ✗ Wrong\r\n\r\n");
                                buffer.push_str("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                                let _ = write!(buffer, "  Style:      {}\r\n", app.style_name);
                                let _ = write!(buffer, "  Answer:     {}\r\n", app.hint);
                                let _ = write!(buffer, "  You typed:  {}\r\n\r\n", app.input);
                                let _ = write!(buffer, "  Score: {}\r\n\r\n", app.score);
                            }
                        }

                        buffer.push_str("  Press Enter to continue...");

                        execute!(
                            io::stdout(),
                            Clear(ClearType::All),
                            cursor::MoveTo(0, 0),
                            Print(&buffer)
                        )?;

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
