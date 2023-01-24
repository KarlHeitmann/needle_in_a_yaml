use serde_yaml; // 0.8.7
use serde_yaml::Value;
use crossterm::terminal::enable_raw_mode;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use crossterm::terminal::disable_raw_mode;
use std::io;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let f = std::fs::File::open("en.yml")?;
    let f = std::fs::File::open("sample.yml")?;
    // let d: String = serde_yaml::from_reader(f)?;
    let d: Value = serde_yaml::from_reader(f)?;
    // println!("Read YAML string: {}", d);

    println!("Read YAML string: {:?}", d);
    // let d = d.unwrap();
    enable_raw_mode().expect("can run in raw mode");

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())

}
