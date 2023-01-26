use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use serde_yaml; // 0.8.7
use serde_yaml::Value;
// use serde_yaml::Value::{Null, Bool, Number, String as YamlString, Sequence, Mapping, Tagged};
use serde_yaml::Value::{Mapping, String as YamlString};

use clap::Parser;

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
    // messages: Value,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    file_path: String,
    // folder: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app, cli.file_path);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn unwrap_value(v: &Value) -> String {
    match serde_yaml::to_string(&v) {
        Ok(s) => s,
        Err(m) => format!("An exception occured: {}", m)
    }
}

fn filter_yaml(v: &Value, ss: &Vec<String>) -> Result<Value, String> {
    let mut founded;
    let mut curr_v = v.clone();
    for s in ss {
        founded = match curr_v {
            Mapping(ref m) => {
                let found_val = m.iter().find(|k| {
                    let my_s = k.0.as_str().unwrap();
                    my_s == s
                } ); //.unwrap().1;
                if found_val.is_none() { return Err(String::from("Uh oh, key not found")) }
                Some(found_val.unwrap().1.clone())
            },
            _ => None
        };
        if founded.is_none() { return Err(String::from("Uh Oh, key not found")); }
        curr_v = founded.unwrap();
    }
    /*
    match Value {
        Null =>,
        Bool(bool),
        Number(Number),
        String(String),
        Sequence(Sequence),
        Mapping(Mapping),
        Tagged(Box<TaggedValue>),
    }
    */
    Ok(curr_v)
}

fn get_keys(tree: &Value) -> Vec<String> { // TARGET
    match tree {
        Mapping(m) => {
            m.iter().map(|k| String::from(k.0.as_str().unwrap())).collect()
        },
        _ => vec![]
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App, file: String) -> Result<(), Box<dyn std::error::Error>> {

    let f = std::fs::File::open(file)?;
    let d: Value = serde_yaml::from_reader(f)?;
    // let filtered_d: Value = d.clone();

    let mut current_tree: Result<Value, String> = Ok(d.clone());
    // let mut filtered_string_representation = my_string_representation.clone();

    loop {
        terminal.draw(|f| ui(f, &app, &current_tree))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let m = app.input.drain(..).collect();
                        // app.input.dr
                        app.messages.push(m);
                        current_tree = filter_yaml(&d, &app.messages);
                    }
                    KeyCode::Delete => {
                        // app.input.push_str(&String::from("ASD"));
                        app.messages.pop();
                        current_tree = filter_yaml(&d, &app.messages);
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc|KeyCode::F(2) => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App, current_tree: &Result<Value, String>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" or "),
                Span::styled("F2", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]); // TARGET
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor( // TARGET
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }

    // let messages: Vec<ListItem> = app
    // let messages: Vec<ListItem> = get_keys(app.messages).into()
    let current_values = match current_tree {
        Ok(t) => get_keys(t),
        // Err(s) => YamlString(String::from(s))
        Err(s) => vec![String::from(s)]
    };
    let messages: Vec<ListItem> = current_values
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Keys at current level"));
    let my_string_representation = match current_tree {
        Ok(v) => unwrap_value(v),
        Err(s) => s.to_string(),
    };

    let viewport = Paragraph::new(my_string_representation); // TARGET

    let nodes_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),
        )
        .split(chunks[2]);
    f.render_widget(viewport, nodes_chunks[0]);
    f.render_widget(messages, nodes_chunks[1]);
    let mut breadcrumbs = app.messages.join(" - ");
    let ruby_code = format!("\nI18n.t(\"{}\")", app.messages.join("."));
    // let status_bar = Paragraph::new(app.messages.join(" - "));
    // let status_bar = Paragraph::new(app.messages.iter().fold(String::new(), |acc, m| format!("{acc} - {m}")));
    breadcrumbs.push_str(&ruby_code);

    // let status_bar = Paragraph::new(breadcrumbs.push_str(&ruby_code));
    let status_bar = Paragraph::new(breadcrumbs);

    f.render_widget(status_bar, chunks[3]);

}


