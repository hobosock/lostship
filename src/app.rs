use crate::{
    gamerules::{
        combat::Combat,
        game_functions::{assess_threat, leap_into_system, JumpStep},
        pilot::Pilot,
        ship::{Scout, Status, SubSystem},
        threat::Threats,
        Leap,
    },
    tui::*,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::TableState};
use std::io;

// define the app
#[derive(Debug)]
pub struct App {
    pub active_tab: MenuTabs,
    pub exit: bool,
    pub name: String,
    pub leaps_since_incident: u64,
    pub fuel: u64,
    pub parts: u64,
    pub hull_damage: u64,
    pub hull_upgrade: bool,
    pub hull_destroyed: bool,
    pub engine: SubSystem,
    pub mining_laser: SubSystem,
    pub scout_bay: SubSystem,
    pub sick_bay: SubSystem,
    pub sensors: SubSystem,
    pub scouts: [Scout; 6],
    pub log: Vec<Leap>,
    pub pilots: [Pilot; 6],
    pub in_combat: bool,
    pub combat: Option<Combat>,
    pub game_text: String,
    pub jump_step: JumpStep,
    pub hanger_state: TableState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_tab: MenuTabs::default(),
            exit: false,
            name: "Lost Ship".to_string(),
            leaps_since_incident: 0,
            fuel: 6,
            parts: 6,
            hull_damage: 0,
            hull_upgrade: false,
            hull_destroyed: false,
            engine: SubSystem::default(),
            mining_laser: SubSystem::default(),
            scout_bay: SubSystem::default(),
            sick_bay: SubSystem::default(),
            sensors: SubSystem::default(),
            scouts: [
                Scout::default(),
                Scout::default(),
                Scout::default(),
                Scout::default(),
                Scout::default(),
                Scout::default(),
            ],
            log: vec![Leap::default()],
            pilots: [
                Pilot::default(),
                Pilot::default(),
                Pilot::default(),
                Pilot::default(),
                Pilot::default(),
                Pilot::default(),
            ],
            in_combat: false,
            combat: None,
            game_text: "".to_string(),
            jump_step: JumpStep::Step1,
            hanger_state: TableState::default(),
        }
    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// render the frame
    fn render_frame(&mut self, frame: &mut Frame) {
        ui(frame, self);
    }

    /// update's application state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    /// handle key events
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('1') => self.active_tab = MenuTabs::Status,
            KeyCode::Char('2') => self.active_tab = MenuTabs::Log,
            KeyCode::Char('3') => self.active_tab = MenuTabs::Hangar,
            KeyCode::Char('4') => self.active_tab = MenuTabs::Crew,
            KeyCode::Char('5') => self.active_tab = MenuTabs::Combat,
            KeyCode::Char('6') => self.active_tab = MenuTabs::About,
            KeyCode::Char('7') => self.active_tab = MenuTabs::Help,
            KeyCode::Char('n') => n_key_press(self),
            KeyCode::Up => up_press(self),
            KeyCode::Down => down_press(self),
            _ => {}
        }
    }

    /// app methods
    fn exit(&mut self) {
        self.exit = true;
    }
}

/// logic for up arrow key presses
/// adjusts table selection up with wrapping on Hangar/Crew/Combat tabs
fn up_press(app: &mut App) {
    match app.active_tab {
        MenuTabs::Hangar => app
            .hanger_state
            .select(select_up(app.hanger_state.selected(), app.scouts.len())),
        _ => {}
    }
}

/// logic for down arrow key presses
/// adjusts table selection down with wrapping on Hangar/Crew/Combat tabs
fn down_press(app: &mut App) {
    match app.active_tab {
        MenuTabs::Hangar => app
            .hanger_state
            .select(select_down(app.hanger_state.selected(), app.scouts.len())),
        _ => {}
    }
}

/// logic for n key presses
/// only active on Status tab, advances one step at a time and waits for combat to resolve
fn n_key_press(app: &mut App) {
    if app.active_tab == MenuTabs::Status {
        match app.jump_step {
            JumpStep::Step1 => {
                app.game_text += "Jumping into a new system ...";
                leap_into_system(app);
                app.jump_step = JumpStep::Step2;
            }
            JumpStep::Step2 => {
                app.game_text = "Assessing threats ...".to_string();
                let scout_vec = Vec::from(app.scouts.clone());
                let enemy_vec = match assess_threat(app) {
                    Some(ev) => {
                        app.game_text += "Enemy ships are preparing to engage!";
                        app.in_combat = true;
                        ev
                    }
                    None => {
                        app.game_text += "Sector clear.  Whew!";
                        app.in_combat = false;
                        vec![Threats::None]
                    }
                };
                app.combat = Some(Combat {
                    rounds: 1,
                    scout_turns: vec![true; scout_vec.len()],
                    scout_formation: scout_vec,
                    enemy_turns: vec![true; enemy_vec.len()],
                    enemy_formation: enemy_vec,
                    scout_half: true,
                });
                app.jump_step = JumpStep::Step3;
            }
            JumpStep::Step3 => {}
            JumpStep::Step4 => {}
            JumpStep::Step5 => {}
            JumpStep::Step6 => {}
            JumpStep::Step7 => {}
        }
    }
}
