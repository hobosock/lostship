use crossterm::{execute, terminal::*};
use ratatui::{
    prelude::*,
    style::Stylize,
    symbols::border,
    widgets::{block::*, *},
};
use std::io::{self, stdout, Stdout};

use crate::{
    app::App,
    gamerules::{
        pilot::{PilotStatus, Rank},
        ship::ShipDamage,
    },
    resources::{about::ABOUT_STR, help::HELP_STR},
};

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
            let main_thing = Paragraph::new(main_text).block(main_block);
            frame.render_widget(main_thing, chunks[1]);
        }
        MenuTabs::Log => {}
        MenuTabs::Hangar => {
            draw_main_hangar_tab(app, frame, chunks[1], main_block);
        }
        MenuTabs::Crew => {
            draw_main_crew_tab(app, frame, chunks[1], main_block);
        }
        MenuTabs::About => {
            draw_main_about_tab(frame, chunks[1], main_block);
        }
        MenuTabs::Help => {
            draw_main_help_tab(frame, chunks[1], main_block);
            instructions_text = Text::from(vec![Line::from(vec![
                "<Q>".yellow().bold(),
                " Quit ".into(),
                "<1-6>".yellow().bold(),
                " Change Tab".into(),
            ])]);
        }
    };

    let instructions = Paragraph::new(instructions_text)
        .centered()
        .block(instructions_block);

    // render
    frame.render_widget(tabs, chunks[0]);
    frame.render_widget(instructions, chunks[2]);
}

fn draw_main_log_tab(app: &App) {
    // table with a row for each leap?
}

fn draw_main_hangar_tab(app: &App, frame: &mut Frame, chunk: Rect, main_block: Block) {
    let header_row = Row::new(vec!["Flight Position", "Ship Name", "Pilot", "Damage"])
        .style(Style::default().cyan().bold())
        .bottom_margin(1);
    let mut rows = [
        Row::default(),
        Row::default(),
        Row::default(),
        Row::default(),
        Row::default(),
        Row::default(),
    ];
    for (i, scout) in app.scouts.iter().enumerate() {
        let damage_text = match scout.ship.damage {
            ShipDamage::Normal => scout.ship.damage.to_string().green(),
            ShipDamage::Half => scout.ship.damage.to_string().yellow(),
            ShipDamage::Inoperable => scout.ship.damage.to_string().red(),
            ShipDamage::Destroyed => scout.ship.damage.to_string().red().underlined(),
        };
        let pilot_text = match scout.pilot.rank {
            Rank::Rookie => scout.pilot.name.clone().white(),
            Rank::Veteran => format!("|V| {}", scout.pilot.name).blue(),
            Rank::Ace => format!("|A| {}", scout.pilot.name).green(),
        };
        let row = Row::new(vec![
            Cell::from(scout.position.to_string()),
            Cell::from(scout.ship.name.clone()),
            Cell::from(pilot_text),
            Cell::from(damage_text),
        ]);
        rows[i] = row;
    }
    let widths = [
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ];
    let table = Table::new(rows, widths)
        .column_spacing(1)
        .header(header_row)
        .block(main_block)
        .highlight_style(Style::default().reversed())
        .highlight_symbol(">>");
    frame.render_widget(table, chunk);
}

/// renders the main block for the Crew tab
fn draw_main_crew_tab(app: &App, frame: &mut Frame, chunk: Rect, main_block: Block) {
    let header_row = Row::new(vec!["Name", "Kills", "Rank", "Status", "Leaps Injured"])
        .style(Style::default().cyan().bold())
        .bottom_margin(1);
    let mut rows = [
        Row::default(),
        Row::default(),
        Row::default(),
        Row::default(),
        Row::default(),
        Row::default(),
    ];
    for (i, pilot) in app.pilots.iter().enumerate() {
        let rank_text = match pilot.rank {
            Rank::Rookie => pilot.rank.to_string().white(),
            Rank::Veteran => pilot.rank.to_string().cyan(),
            Rank::Ace => pilot.rank.to_string().green(),
        };
        let injured_text = match pilot.status {
            PilotStatus::Normal => pilot.status.to_string().green(),
            PilotStatus::Injured => pilot.status.to_string().yellow(),
            PilotStatus::KIA => pilot.status.to_string().red(),
        };
        // TODO: color leaps injured row?
        let row = Row::new(vec![
            Cell::from(pilot.name.clone()),
            Cell::from(pilot.kills.to_string()),
            Cell::from(rank_text),
            Cell::from(injured_text),
            Cell::from(pilot.injury_timer.to_string()),
        ]);
        rows[i] = row;
    }
    let widths = [
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ];
    let table = Table::new(rows, widths)
        .column_spacing(1)
        .header(header_row)
        .block(main_block)
        .highlight_style(Style::default().reversed())
        .highlight_symbol(">>");
    frame.render_widget(table, chunk);
}

/// renders main block for About tab
fn draw_main_about_tab(frame: &mut Frame, chunk: Rect, main_block: Block) {
    let paragraph = Paragraph::new(ABOUT_STR)
        .wrap(Wrap { trim: false })
        .block(main_block);
    frame.render_widget(paragraph, chunk);
}

/// renders main block for Help tab
fn draw_main_help_tab(frame: &mut Frame, chunk: Rect, main_block: Block) {
    let paragraph = Paragraph::new(HELP_STR)
        .wrap(Wrap { trim: false })
        .block(main_block);
    frame.render_widget(paragraph, chunk);
}
