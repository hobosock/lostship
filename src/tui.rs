use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::Layout,
    prelude::*,
    style::Stylize,
    symbols::border,
    widgets::{
        block::{Block, Position, Title},
        Borders, Cell, List, Paragraph, Row, Table, Tabs, Wrap,
    },
};
use std::io::{self, stdout, Stdout};

use crate::{
    app::App,
    gamerules::{
        combat::combat_to_app,
        game_functions::JumpStep,
        pilot::{PilotStatus, Rank},
        ship::ShipDamage,
    },
    resources::{about::ABOUT_STR, help::HELP_STR},
};

/// a type alias for the terminal type used
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// tabs for main TUI interface
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum MenuTabs {
    #[default]
    Status,
    Log,
    Hangar,
    Crew,
    Combat,
    About,
    Help,
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
pub fn ui(frame: &mut Frame, app: &mut App) {
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
        "5. Combat",
        "6. About",
        "7. Help",
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

    // change bottom two chunks based on selected tab
    match app.active_tab {
        MenuTabs::Status => {
            draw_main_status_tab(app, frame, chunks[1], main_block);
            instructions_text = Text::from(vec![Line::from(vec![
                "<Q>".yellow().bold(),
                " Quit ".into(),
                "<Up>/<Down>".yellow().bold(),
                " Change selection. ".into(),
                "<R>".yellow().bold(),
                " Repair ".into(),
            ])]);
        }
        MenuTabs::Log => {}
        MenuTabs::Hangar => {
            draw_main_hangar_tab(app, frame, chunks[1], main_block);
            instructions_text = Text::from(vec![Line::from(vec![
                "<Q>".yellow().bold(),
                " Quit ".into(),
                "<Up>/<Down>".yellow().bold(),
                " Change selection. ".into(),
                "<E>".yellow().bold(),
                " Edit ".into(),
                "<W>/<S>".yellow().bold(),
                " Shift Assignment ".into(),
                "<R>".yellow().bold(),
                " Repair ".into(),
            ])]);
        }
        MenuTabs::Crew => {
            draw_main_crew_tab(app, frame, chunks[1], main_block);
            instructions_text = Text::from(vec![Line::from(vec![
                "<Q>".yellow().bold(),
                " Quit ".into(),
                "<Up>/<Down>".yellow().bold(),
                " Change selection. ".into(),
                "<E>".yellow().bold(),
                " Edit ".into(),
                "<W>/<S>".yellow().bold(),
                " Shift Assignment ".into(),
            ])]);
        }
        MenuTabs::Combat => {
            draw_main_combat_tab(app, frame, chunks[1], main_block);
            instructions_text = Text::from(vec![Line::from(vec![
                "<Q>".yellow().bold(),
                " Quit ".into(),
                "<Up>/<Down>".yellow().bold(),
                " Change selection ".into(),
                "<Left>/<Right>".yellow().bold(),
                " Change Table ".into(),
                "<A>".yellow().bold(),
                " Scout Attack ".into(),
                "<M>".yellow().bold(),
                " Mining Laser ".into(),
            ])]);
        }
        MenuTabs::About => {
            draw_main_about_tab(frame, chunks[1], main_block);
        }
        MenuTabs::Help => {
            draw_main_help_tab(frame, chunks[1], main_block);
            instructions_text = Text::from(vec![Line::from(vec![
                "<Q>".yellow().bold(),
                " Quit ".into(),
                "<1-7>".yellow().bold(),
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

    // draw editing popup
    if app.editing {
        let popup_block = Block::default()
            .title("Enter Name")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));
        let popup_area = centered_rect(frame.size(), 30, 10);
        let edit_paragraph = Paragraph::new(app.edit_string.clone()).block(popup_block);
        frame.render_widget(edit_paragraph, popup_area);
    }
}

/// draws center chunk of Status tab
fn draw_main_status_tab(app: &mut App, frame: &mut Frame, chunk: Rect, main_block: Block) {
    let inner_area = main_block.inner(chunk);
    main_block.render(chunk, frame.buffer_mut());
    // TODO: needs some padding and stuff
    let sub_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(inner_area);

    // status, left section
    // TODO: change color based on number, status
    let status_text = Text::from(vec![
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
        Line::from(vec![app.game_text.as_str().into()]),
    ]);
    let main_thing = Paragraph::new(status_text);
    frame.render_widget(main_thing, sub_chunks[0]);

    // sub system list, right section
    let list_items = [
        "Hull",
        "Engines",
        "Mining Laser",
        "Scout Bay",
        "Sick Bay",
        "Sensors",
    ];
    let list = List::new(list_items)
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);
    frame.render_stateful_widget(list, sub_chunks[1], &mut app.subsys_list_state);
}

fn draw_main_log_tab(app: &mut App) {
    // table with a row for each leap?
}

fn draw_main_hangar_tab(app: &mut App, frame: &mut Frame, chunk: Rect, main_block: Block) {
    // reset pilot information in case order changed
    for i in 0..app.scouts.len() {
        app.scouts[i].pilot = app.pilots[i].clone();
    }
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
    frame.render_stateful_widget(table, chunk, &mut app.hanger_state);
}

/// renders the main block for the Crew tab
fn draw_main_crew_tab(app: &mut App, frame: &mut Frame, chunk: Rect, main_block: Block) {
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
            PilotStatus::Kia => pilot.status.to_string().red(),
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
    frame.render_stateful_widget(table, chunk, &mut app.crew_state);
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

/// renders the main block for Combat tab - different depending on in combat or not
fn draw_main_combat_tab(app: &mut App, frame: &mut Frame, chunk: Rect, main_block: Block) {
    let inner_area = main_block.inner(chunk);
    main_block.render(chunk, frame.buffer_mut());
    if app.in_combat && app.combat.is_some() {
        let mut combat = app.combat.clone().unwrap();

        // check if combat is resolved
        if combat.enemy_stats.iter().all(|x| x.hp == 0 && x.fuel == 0) {
            app.combat = None;
            app.in_combat = false;
            app.jump_step = JumpStep::Step4;
        }

        // reset pilot information in case order changed
        // TODO: maybe don't want to do this during a combat phase?
        for i in 0..app.scouts.len() {
            app.scouts[i].pilot = app.pilots[i].clone();
        }

        if combat.rounds == 1 {
            combat.laser_fired = true;
        }

        // TODO: should this be mode dependent?
        // check to see if all of scouts have taken a turn
        if combat.scout_turns.iter().all(|x| *x) && combat.laser_fired {
            combat.scout_half = false; // now enemy turn
            combat.scout_turns = vec![false; combat.scout_formation.len()]; // reset
        }
        if combat.enemy_turns.iter().all(|x| *x) {
            combat.scout_half = true;
            combat.enemy_turns = vec![false; combat.enemy_formation.len()];
            combat.laser_fired = false;
            // end of round, +/- fuel, round counter, etc.
            combat.rounds += 1;
            for (i, enemy) in app.combat.as_ref().unwrap().enemy_stats.iter().enumerate() {
                if enemy.fuel > 0 {
                    combat.enemy_stats[i].fuel -= 1;
                }
            }
        }

        // skip turns for Scouts that are inoperable, destroyed, or KIA
        for (i, scout) in combat.scout_formation.iter().enumerate() {
            if scout.ship.damage == ShipDamage::Inoperable
                || scout.ship.damage == ShipDamage::Destroyed
                || scout.pilot.status == PilotStatus::Kia
            {
                combat.scout_turns[i] = true;
            }
        }

        let sub_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(3),
            ])
            .split(inner_area);
        let ship_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(sub_chunks[1]);

        let colony_ship_text = if combat.rounds > 1 {
            "Colony ship in range!"
        } else {
            "Colony ship not yet in range."
        };
        let paragraph = Paragraph::new(format!(
            "Round: {} | {} | out of fuel after X rounds: Mk1 - 3, Mk2 - 4, Mk3 - 5",
            combat.rounds, colony_ship_text
        ));
        frame.render_widget(paragraph, sub_chunks[0]);
        let combat_paragraph = Paragraph::new(combat.combat_text.clone());
        frame.render_widget(combat_paragraph, sub_chunks[2]);

        let ship_border = if app.combat_select {
            Borders::ALL
        } else {
            Borders::NONE
        };
        let enemy_border = if app.combat_select {
            Borders::NONE
        } else {
            Borders::ALL
        };

        let mut rows: Vec<Row> = Vec::new();
        for scout in &combat.scout_formation {
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
            rows.push(Row::new(vec![
                Cell::from(scout.position.to_string()),
                Cell::from(scout.ship.name.clone()),
                Cell::from(pilot_text),
                Cell::from(damage_text),
            ]));
        }
        let header_row = Row::new(vec!["Flight Position", "Ship Name", "Pilot", "Damage"])
            .style(Style::default().cyan().bold())
            .bottom_margin(1);
        let widths = [
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ];
        let scout_table = Table::new(rows, widths)
            .column_spacing(1)
            .header(header_row)
            .block(Block::default().borders(ship_border))
            .highlight_style(Style::default().reversed())
            .highlight_symbol(">>");
        frame.render_stateful_widget(scout_table, ship_chunks[0], &mut app.combat_scout_state);

        let mut rows: Vec<Row> = Vec::new();
        for fighter in &combat.enemy_stats {
            rows.push(Row::new(vec![
                Cell::from(fighter.model.to_string()),
                Cell::from(fighter.guns.to_string()),
                Cell::from(fighter.fuel.to_string()),
                Cell::from(fighter.hp.to_string()),
            ]));
        }
        let header_row = Row::new(vec!["Type", "Guns", "Fuel", "HP"])
            .style(Style::default().cyan().bold())
            .bottom_margin(1);
        let widths = [
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ];
        let enemy_table = Table::new(rows, widths)
            .column_spacing(1)
            .header(header_row)
            .block(Block::default().borders(enemy_border))
            .highlight_style(Style::default().reversed())
            .highlight_symbol(">>");
        frame.render_stateful_widget(enemy_table, ship_chunks[1], &mut app.combat_enemy_state);

        combat_to_app(&combat, app);
        app.combat = Some(combat);
    } else {
        // TODO: somehow wipe combat tab after it's resolved?
        let paragraph = Paragraph::new("Not in combat at the moment - whew!");
        frame.render_widget(paragraph, inner_area);
    }
}

/// convenience function for incrementing table selection (down arrow)
pub fn select_down(current: Option<usize>, length: usize) -> Option<usize> {
    if length < 1 {
        None
    } else if current.is_none() {
        Some(0)
    } else {
        let limit = length - 1;
        if current.unwrap() == limit {
            Some(0)
        } else {
            Some(current.unwrap() + 1)
        }
    }
}
/// convenience function for decrementing table selection (up arrow)
pub fn select_up(current: Option<usize>, length: usize) -> Option<usize> {
    if length < 1 {
        None
    } else if current.is_none() {
        Some(length - 1)
    } else {
        let limit = length - 1;
        if current.unwrap() == 0 {
            Some(limit)
        } else {
            Some(current.unwrap() - 1)
        }
    }
}

/// centers a Rect in current area
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
