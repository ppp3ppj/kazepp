// Ultra Simple Typing Practice
use rand::prelude::IndexedRandom;
// Ultra Simple Typing Practice
use convert_case::{Case, Casing};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    cursor,
};
use rand::{seq::SliceRandom, Rng};
use std::io::{self, Write};

const WORDS: &[&str] = &[
    "user", "name", "email", "password", "database", "connection",
    "function", "method", "class", "variable", "create", "read",
    "update", "delete", "first", "last", "get", "set",
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
        clear_screen()?;

        if !app.got_name {
            // Page 1: Name entry
            print!("\r\n  Typing Practice\r\n\r\n");
            print!("  What is your name?\r\n");
            print!("  > {}", app.username);
            io::stdout().flush()?;
        } else if !app.got_mode {
            // Page 2: Mode selection
            print!("\r\n  Hello, {}!\r\n\r\n", app.username);
            print!("  Select difficulty:\r\n\r\n");
            print!("  1. Normal (with hint)\r\n");
            print!("  2. Hard (no hint - lose resets score!)\r\n\r\n");
            print!("  Press 1 or 2 to select");
            io::stdout().flush()?;
        } else {
            // Page 3: Practice - MUCH CLEARER UI
            print!("\r\n\r\n");
            print!("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
            print!("  Words:  {}\r\n\r\n", app.source);
            print!("  Task:   Convert to {}\r\n\r\n", app.style_name);
            print!("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");

            // Show hint only in Normal mode
            if matches!(app.mode, Some(GameMode::Normal)) {
                print!("  Hint:   {}\r\n\r\n", app.hint);
            }

            print!("  Answer: {}", app.input);
            print!("\r\n\r\n");
            print!("  Score: {}\r\n", app.score);
            print!("\r\n");
            io::stdout().flush()?;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
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

                        clear_screen()?;

                        if correct {
                            // Correct answer
                            app.score += 1;
                            print!("\r\n\r\n");
                            print!("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                            print!("  ✓ CORRECT!\r\n\r\n");
                            print!("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                            print!("  Style:      {}\r\n", app.style_name);
                            print!("  Answer:     {}\r\n", app.hint);
                            print!("  You typed:  {}\r\n\r\n", app.input);
                            print!("  Score: {}\r\n\r\n", app.score);
                        } else {
                            // Wrong answer
                            if matches!(app.mode, Some(GameMode::Hard)) {
                                // Hard mode: YOU LOSE - reset score
                                print!("\r\n\r\n");
                                print!("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                                print!("  ✗ YOU LOSE!\r\n\r\n");
                                print!("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                                print!("  Style:      {}\r\n", app.style_name);
                                print!("  Answer:     {}\r\n", app.hint);
                                print!("  You typed:  {}\r\n\r\n", app.input);
                                print!("  Score reset to 0\r\n\r\n");
                                app.score = 0;
                            } else {
                                // Normal mode: just wrong, keep score
                                print!("\r\n\r\n");
                                print!("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                                print!("  ✗ Wrong\r\n\r\n");
                                print!("  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\r\n\r\n");
                                print!("  Style:      {}\r\n", app.style_name);
                                print!("  Answer:     {}\r\n", app.hint);
                                print!("  You typed:  {}\r\n\r\n", app.input);
                                print!("  Score: {}\r\n\r\n", app.score);
                            }
                        }

                        print!("  Press Enter to continue...");
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
