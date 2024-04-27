// external dependencies
use std::io;

// local inclusions
mod app;
mod gamerules;
mod tui;
use app::App;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
