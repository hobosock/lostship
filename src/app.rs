use crate::{
    gamerules::{
        combat::{enemy_damage, scout_attack, Combat},
        game_functions::{assess_threat, leap_into_system, search_wreckage, system_scan, JumpStep},
        pilot::{Pilot, PilotStatus},
        ship::{Scout, ShipDamage, SubSystem},
        threat::{threats_to_fighters, Threats},
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
    pub pilot_assignment: [usize; 6],
    pub in_combat: bool,
    pub combat: Option<Combat>,
    pub game_text: String,
    pub jump_step: JumpStep,
    pub hanger_state: TableState,
    pub crew_state: TableState,
    pub editing: bool,
    pub edit_string: String,
    pub edit_target: Option<usize>,
    pub combat_select: bool, // indicates active table in Combat tab
    pub combat_scout_state: TableState,
    pub combat_enemy_state: TableState,
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
            pilot_assignment: [0, 1, 2, 3, 4, 5],
            in_combat: false,
            combat: None,
            game_text: "".to_string(),
            jump_step: JumpStep::Step1,
            hanger_state: TableState::default(),
            crew_state: TableState::default(),
            editing: false,
            edit_string: String::new(),
            edit_target: None,
            combat_select: true,
            combat_scout_state: TableState::default(),
            combat_enemy_state: TableState::default(),
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
        if self.editing {
            match key_event.code {
                KeyCode::Esc => {
                    self.editing = false;
                    self.edit_string = String::new();
                }
                KeyCode::Enter => enter_press(self),
                KeyCode::Backspace => {
                    self.edit_string.pop();
                }
                KeyCode::Char(value) => {
                    self.edit_string.push(value);
                }
                _ => {}
            }
        } else {
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
                KeyCode::Char('e') => edit_press(self),
                KeyCode::Char('w') => w_key_press(self),
                KeyCode::Char('s') => s_key_press(self),
                KeyCode::Char('a') => a_key_press(self),
                KeyCode::Up => up_press(self),
                KeyCode::Down => down_press(self),
                KeyCode::Left => left_press(self),
                KeyCode::Right => right_press(self),
                _ => {}
            }
        }
    }

    /// app methods
    fn exit(&mut self) {
        self.exit = true;
    }
}

/// handles 'e' edit key
/// gets a reference to the name field of whatever selected table element
/// enables edit mode and clears previous string just in case
fn edit_press(app: &mut App) {
    match app.active_tab {
        MenuTabs::Hangar => {
            if app.hanger_state.selected().is_some() {
                app.editing = true;
                app.edit_string = String::new();
                app.edit_target = Some(app.hanger_state.selected().unwrap());
            }
        }
        MenuTabs::Crew => {
            if app.crew_state.selected().is_some() {
                app.editing = true;
                app.edit_string = String::new();
                app.edit_target = Some(app.crew_state.selected().unwrap());
            }
        }
        _ => {}
    }
}

fn enter_press(app: &mut App) {
    match app.active_tab {
        MenuTabs::Hangar => {
            if app.edit_target.is_some() {
                app.scouts[app.edit_target.unwrap()].ship.name = app.edit_string.clone();
            }
        }
        MenuTabs::Crew => {
            if app.edit_target.is_some() {
                app.pilots[app.edit_target.unwrap()].name = app.edit_string.clone();
            }
        }
        _ => {}
    }
    app.editing = false;
}

/// logic for up arrow key presses
/// adjusts table selection up with wrapping on Hangar/Crew/Combat tabs
fn up_press(app: &mut App) {
    match app.active_tab {
        MenuTabs::Hangar => app
            .hanger_state
            .select(select_up(app.hanger_state.selected(), app.scouts.len())),
        MenuTabs::Crew => app
            .crew_state
            .select(select_up(app.crew_state.selected(), app.pilots.len())),
        MenuTabs::Combat => {
            if app.combat.is_some() {
                if app.combat_select {
                    app.combat_scout_state.select(select_up(
                        app.combat_scout_state.selected(),
                        app.scouts.len(),
                    ));
                } else {
                    app.combat_enemy_state.select(select_up(
                        app.combat_enemy_state.selected(),
                        app.combat.as_ref().unwrap().enemy_formation.len(),
                    ));
                }
            }
        }
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
        MenuTabs::Crew => app
            .crew_state
            .select(select_down(app.crew_state.selected(), app.pilots.len())),
        MenuTabs::Combat => {
            if app.combat.is_some() {
                if app.combat_select {
                    app.combat_scout_state.select(select_down(
                        app.combat_scout_state.selected(),
                        app.scouts.len(),
                    ));
                } else {
                    app.combat_enemy_state.select(select_down(
                        app.combat_enemy_state.selected(),
                        app.combat.as_ref().unwrap().enemy_formation.len(),
                    ));
                }
            }
        }
        _ => {}
    }
}

/// logic for left arrow presses
fn left_press(app: &mut App) {
    match app.active_tab {
        MenuTabs::Combat => app.combat_select = true,
        _ => {}
    }
}

/// logic for right arrow presses
fn right_press(app: &mut App) {
    match app.active_tab {
        MenuTabs::Combat => app.combat_select = false,
        _ => {}
    }
}

/// shifts position of selected element up (swaps with n-1 element with wrapping)
fn shift_up<T: Clone>(v: &mut [T], pos: usize) {
    let max_pos = v.len() - 1;
    let b = if pos == 0 {
        v[max_pos].clone()
    } else {
        v[pos - 1].clone()
    };
    let a = v[pos].clone();
    v[pos] = b;
    if pos == 0 {
        v[max_pos] = a;
    } else {
        v[pos - 1] = a;
    }
}

/// shifts position of selected element down (swaps with n+1 element with wrapping)
fn shift_down<T: Clone>(v: &mut [T], pos: usize) {
    let max_pos = v.len() - 1;
    let b = if pos == max_pos {
        v[0].clone()
    } else {
        v[pos + 1].clone()
    };
    let a = v[pos].clone();
    v[pos] = b;
    if pos == max_pos {
        v[0] = a;
    } else {
        v[pos + 1] = a;
    }
}

/// logic for w key presses
fn w_key_press(app: &mut App) {
    match app.active_tab {
        MenuTabs::Crew => {
            if app.crew_state.selected().is_some() {
                shift_up(&mut app.pilots, app.crew_state.selected().unwrap());
                app.crew_state
                    .select(select_up(app.crew_state.selected(), app.pilots.len()));
            }
        }
        MenuTabs::Hangar => {
            if app.hanger_state.selected().is_some() {
                shift_up(&mut app.scouts, app.hanger_state.selected().unwrap());
                shift_up(&mut app.pilots, app.hanger_state.selected().unwrap());
                app.hanger_state
                    .select(select_up(app.hanger_state.selected(), app.scouts.len()));
            }
        }
        _ => {}
    }
}

/// logic for s key presses
fn s_key_press(app: &mut App) {
    match app.active_tab {
        MenuTabs::Crew => {
            if app.crew_state.selected().is_some() {
                shift_down(&mut app.pilots, app.crew_state.selected().unwrap());
                app.crew_state
                    .select(select_down(app.crew_state.selected(), app.pilots.len()));
            }
        }
        MenuTabs::Hangar => {
            if app.hanger_state.selected().is_some() {
                shift_down(&mut app.scouts, app.hanger_state.selected().unwrap());
                shift_down(&mut app.pilots, app.hanger_state.selected().unwrap());
                app.hanger_state
                    .select(select_down(app.hanger_state.selected(), app.scouts.len()));
            }
        }
        _ => {}
    }
}

/// logic for a key presses
/// if in combat AND scout turn AND selected valid scout AND enemy, roll for damage
fn a_key_press(app: &mut App) {
    if app.combat.is_some()
        && app.combat.as_ref().unwrap().scout_half
        && app.combat_scout_state.selected().is_some()
        && app.combat_enemy_state.selected().is_some()
    {
        // make sure valid ships are selected (not destroyed, etc.)
        let mut combat = app.combat.clone().unwrap();
        let scout_pos = app.combat_scout_state.selected().unwrap();
        let scout = app.scouts[scout_pos].clone();
        let turn_ok = combat.scout_turns[scout_pos];
        let enemy_pos = app.combat_enemy_state.selected().unwrap();
        let enemy = combat.enemy_stats[enemy_pos].clone();
        let ship_ok = match scout.ship.damage {
            ShipDamage::Normal => true,
            ShipDamage::Half => true,
            _ => false,
        };
        let pilot_ok = match scout.pilot.status {
            PilotStatus::Normal => true,
            PilotStatus::Injured => true,
            _ => false,
        };
        let target_ok = if enemy.fuel > 0 && enemy.hp > 0 {
            true
        } else {
            false
        };

        if ship_ok && pilot_ok && target_ok && !turn_ok {
            let damage = scout_attack(&scout);
            combat.enemy_stats[enemy_pos].hp = enemy_damage(damage, enemy.hp);
            combat.scout_turns[scout_pos] = true;
        }
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
                    enemy_stats: threats_to_fighters(&enemy_vec),
                    enemy_formation: enemy_vec,
                    scout_half: true,
                });
                app.jump_step = JumpStep::Step3;
            }
            JumpStep::Step3 => {
                // TODO: just create a battle for combat testing for now
                app.combat = Some(Combat {
                    rounds: 0,
                    scout_formation: app.scouts.to_vec(),
                    enemy_formation: vec![Threats::Mk1, Threats::Mk2],
                    enemy_stats: threats_to_fighters(&vec![Threats::Mk1, Threats::Mk2]),
                    scout_turns: vec![false; 6],
                    enemy_turns: vec![false, false],
                    scout_half: true,
                });
                app.in_combat = true;
            }
            JumpStep::Step4 => {
                // TODO: error proof
                app.parts += search_wreckage(app.combat.clone().unwrap().enemy_formation);
                app.jump_step = JumpStep::Step5;
            }
            JumpStep::Step5 => {
                let (fuel, scan_result) = system_scan(app.leaps_since_incident);
                app.fuel += fuel;
                app.jump_step = JumpStep::Step6;
            }
            JumpStep::Step6 => {}
            JumpStep::Step7 => {}
        }
    }
}
