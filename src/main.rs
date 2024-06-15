// external dependencies
use std::io;

// local inclusions
mod app;
mod gamerules;
mod resources;
mod tui;
use crate::tui::interface_core::{init, restore};
use app::App;

fn main() -> io::Result<()> {
    let mut terminal = init()?;
    let app_result = App::default().run(&mut terminal);
    restore()?;
    app_result
}
