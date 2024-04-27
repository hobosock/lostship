use crossterm::{execute, terminal::*};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::io::{self, stdout, Stdout};

use crate::app::App;

/// a type alias for the terminal type used
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// tabs for main TUI interface
#[derive(Debug, Copy, Clone)]
pub enum MenuTabs {
    Status,
    Log,
    Hangar,
    Crew,
    About,
    Help,
}

impl Default for MenuTabs {
    fn default() -> MenuTabs {
        MenuTabs::Status
    }
}

/// initialize the terminal
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

/// main UI definition
pub fn ui(frame: &mut Frame, app: &App) {
    // split area up into 3 chunks (tabs/main/keys)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(2),
            Constraint::Length(3),
        ])
        .split(frame.size());
    // tabs for switching between menus
    let tabs = Tabs::new(vec![
        "1. Status",
        "2. Log",
        "3. Hangar",
        "4. Crew",
        "5. About",
        "6. Help",
    ])
    .block(Block::default().title("Menu").borders(Borders::ALL))
    .style(Style::default().white())
    .highlight_style(Style::default().cyan().bold())
    .select(app.active_tab as usize);
    // main/center panel for display
    let version = Title::from(Line::from(vec![" Lost Ship v0.1.0 ".into()]));
    let main_block = Block::default()
        .title(
            Title::from(
                Line::from(format!("| {} |", app.name.clone()))
                    .style(Style::default().cyan().bold()),
            )
            .alignment(Alignment::Center)
            .position(Position::Top),
        )
        .title(
            version
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(border::THICK);
    // bottom panel to display keys
    let instructions_block = Block::default().borders(Borders::ALL);
    let mut instructions_text = Text::from(vec![Line::from(vec!["<Q> Quit".into()])]);
    let mut main_text = Text::from(vec![Line::from(vec!["Placeholder".into()])]);

    // change bottom two chunks based on selected tab
    match app.active_tab {
        MenuTabs::Status => {
            // TODO: change color based on number, status
            main_text = Text::from(vec![
                Line::from(vec![
                    "LEAPS SINCE INCIDENT: ".into(),
                    app.leaps_since_incident.to_string().into(),
                ]),
                Line::from(vec!["Fuel: ".into(), app.fuel.to_string().into()]),
                Line::from(vec!["Parts: ".into(), app.parts.to_string().into()]),
                Line::from(vec![
                    "Hull Damage: ".into(),
                    app.hull_damage.to_string().into(),
                ]),
                Line::from(vec![
                    "Engines: ".into(),
                    format!("{}", app.engine.status).into(),
                ]),
                Line::from(vec![
                    "Mining Laser: ".into(),
                    format!("{}", app.mining_laser.status).into(),
                ]),
                Line::from(vec![
                    "Scout Bay: ".into(),
                    format!("{}", app.scout_bay.status).into(),
                ]),
                Line::from(vec![
                    "Sick Bay: ".into(),
                    format!("{}", app.sick_bay.status).into(),
                ]),
                Line::from(vec![
                    "Sensors: ".into(),
                    format!("{}", app.sensors.status).into(),
                ]),
            ]);
        }
        MenuTabs::Log => {}
        MenuTabs::Hangar => {}
        MenuTabs::Crew => {}
        MenuTabs::About => {}
        MenuTabs::Help => {
            instructions_text = Text::from(vec![Line::from(vec![
                "<Q> Quit".into(),
                "<1-6> Change Tab".into(),
            ])]);
        }
    };

    let instructions = Paragraph::new(instructions_text)
        .centered()
        .block(instructions_block);
    let main_thing = Paragraph::new(main_text).block(main_block);

    // render
    frame.render_widget(tabs, chunks[0]);
    frame.render_widget(main_thing, chunks[1]);
    frame.render_widget(instructions, chunks[2]);
}
