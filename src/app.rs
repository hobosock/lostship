use crate::{
    gamerules::{
        combat::{enemy_damage, enemy_turn, mining_laser, scout_attack, Combat},
        game_functions::{assess_threat, leap_into_system, search_wreckage, system_scan, JumpStep},
        pilot::{Pilot, PilotStatus},
        scout::scout_repair,
        ship::{subsystem_repair, Scout, ShipDamage, SubSystem},
        threat::{threats_to_fighters, Threats},
        Leap,
    },
    tui::{select_down, select_up, ui, MenuTabs, Tui},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{ListState, TableState},
};
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
    pub subsys_list_state: ListState,
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
            game_text: String::new(),
            jump_step: JumpStep::Step1,
            hanger_state: TableState::default(),
            crew_state: TableState::default(),
            editing: false,
            edit_string: String::new(),
            edit_target: None,
            combat_select: true,
            combat_scout_state: TableState::default(),
            combat_enemy_state: TableState::default(),
            subsys_list_state: ListState::default(),
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
                self.handle_key_event(key_event);
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
                KeyCode::Char('m') => m_key_press(self),
                KeyCode::Char('r') => r_key_press(self),
                KeyCode::Char('u') => u_key_press(self),
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
                app.scouts[app.edit_target.unwrap()]
                    .ship
                    .name
                    .clone_from(&app.edit_string);
            }
        }
        MenuTabs::Crew => {
            if app.edit_target.is_some() {
                app.pilots[app.edit_target.unwrap()]
                    .name
                    .clone_from(&app.edit_string);
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
        MenuTabs::Status => {
            app.subsys_list_state
                .select(select_up(app.subsys_list_state.selected(), 6));
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
        MenuTabs::Status => {
            app.subsys_list_state
                .select(select_down(app.subsys_list_state.selected(), 6));
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
        let ship_ok = matches!(scout.ship.damage, ShipDamage::Normal | ShipDamage::Half);
        let pilot_ok = matches!(
            scout.pilot.status,
            PilotStatus::Normal | PilotStatus::Injured
        );
        let target_ok = enemy.fuel > 0 && enemy.hp > 0; // bool literal?

        if ship_ok && pilot_ok && target_ok && !turn_ok {
            let damage = scout_attack(&scout);
            combat.enemy_stats[enemy_pos].hp = enemy_damage(damage, enemy.hp);
            combat.scout_turns[scout_pos] = true;
            combat.combat_text = format!(
                "{} deals {} damage to {}",
                scout.pilot.name, damage, enemy.model
            );
        } else {
            combat.combat_text = "Make sure a valid scout and target are selected.".to_string();
        }

        app.combat = Some(combat); // rewrap and assign to app state
    }
}

/// logic for m key press
/// if in combat past first round, triggers mining laser attack on selected enemy
fn m_key_press(app: &mut App) {
    if app.combat.is_some()
        && app.combat.as_ref().unwrap().scout_half
        && app.combat_enemy_state.selected().is_some()
        && !app.combat.as_ref().unwrap().laser_fired
    {
        let mut combat = app.combat.clone().unwrap();
        let enemy_pos = app.combat_enemy_state.selected().unwrap();
        let enemy = combat.enemy_stats[enemy_pos].clone();
        let target_ok = enemy.fuel > 0 && enemy.hp > 0;
        if target_ok && combat.rounds > 1 {
            let damage = mining_laser(app.mining_laser.upgrade);
            combat.enemy_stats[enemy_pos].hp = enemy_damage(damage, enemy.hp);
            combat.laser_fired = true;
            combat.combat_text = format!("Mining laser deals {} damage to {}", damage, enemy.model);
        } else {
            combat.combat_text = "Mining laser available starting in round 2.  Make sure a valid target is selected.".to_string();
        }
        app.combat = Some(combat);
    }
}

/// logic for r key presses
/// results depend on tab and selection - only available in step 6
/// repairs scout damage, consuming parts (50% damage is repaired for free!)
/// repairs sub systems when selected
fn r_key_press(app: &mut App) {
    if app.jump_step == JumpStep::Step6 {
        match app.active_tab {
            MenuTabs::Hangar => {
                if app.hanger_state.selected().is_some() {
                    scout_repair(app, app.hanger_state.selected().unwrap());
                }
                // TODO: notification to select a damaged scout
            }
            MenuTabs::Status => {
                if app.subsys_list_state.selected().is_some() {
                    subsystem_repair(app, app.subsys_list_state.selected().unwrap());
                }
            }
            _ => {}
        }
    }
}

/// logic for u key presses
/// only active on Status tab, upgrades subsystem if not already upgraded and enough parts are
/// available (only works in repair phase)
fn u_key_press(app: &mut App) {
    if app.jump_step == JumpStep::Step6 {
        match app.active_tab {
            MenuTabs::Status => {
                if app.hanger_state.selected().is_some() && app.parts >= 4 {
                    let ss = app.hanger_state.selected().unwrap();
                    if ss == 0 {
                        app.hull_upgrade = true;
                    } else if ss == 1 {
                        app.engine.upgrade = true;
                    } else if ss == 2 {
                        app.mining_laser.upgrade = true;
                    } else if ss == 3 {
                        app.scout_bay.upgrade = true;
                    } else if ss == 4 {
                        app.sick_bay.upgrade = true;
                    } else {
                        app.sensors.upgrade = true;
                    }
                    app.parts -= 4;
                }
            }
            _ => {}
        }
    }
}

/// logic for n key presses
/// only active on Status and Combat tabs, advances one step at a time and waits for combat to resolve
/// on Combat tab, used to advance through enemy turn
fn n_key_press(app: &mut App) {
    match app.active_tab {
        MenuTabs::Status => {
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
                        laser_fired: false,
                        combat_text: "Enemy ships sighted!  Prepare to engage!".to_string(),
                    });
                    app.jump_step = JumpStep::Step3;
                }
                JumpStep::Step3 => {
                    /*
                    app.combat = Some(Combat {
                        rounds: 1,
                        scout_formation: app.scouts.to_vec(),
                        enemy_formation: vec![Threats::Mk1, Threats::Mk2],
                        enemy_stats: threats_to_fighters(&[Threats::Mk1, Threats::Mk2]),
                        scout_turns: vec![false; 6],
                        enemy_turns: vec![false, false],
                        scout_half: true,
                        laser_fired: false,
                        combat_text: "Enemy ships sighted!  Prepare to engage!".to_string(),
                    });
                    app.in_combat = true;
                    */
                    app.jump_step = JumpStep::Step4;
                }
                JumpStep::Step4 => {
                    // TODO: error proof
                    let parts = search_wreckage(&app.combat.clone().unwrap().enemy_formation);
                    app.parts += parts;
                    app.game_text =
                        format!("You search through the wreckage and recover {parts} parts.");
                    app.jump_step = JumpStep::Step5;
                }
                JumpStep::Step5 => {
                    let (fuel, scan_result) = system_scan(app.leaps_since_incident);
                    app.fuel += fuel;
                    app.game_text = format!(
                        "Scanning system... {scan_result} - gathered {fuel} fuel.  Make repairs and upkeep."
                    );
                    // TODO: handle anomoly and home scans
                    app.jump_step = JumpStep::Step6;
                }
                JumpStep::Step6 => {
                    // scouts at 50% are repaired for free
                    // inoperable scouts can be repaired for 1 part
                    // each point of hull damage can be repaired for 1 part
                    // a scout can be scrapped for +4 parts
                    // repairing any system requires 2 parts
                    // upgrading a system costs 4 parts
                    // building a new scout costs 6 parts
                    // after every 5th leap you get a free upgrade
                    // injured pilots heal according to sick bay - do this last
                    // inoperable sick bay means newly injured pilots die
                    // start training up new pilots
                    app.jump_step = JumpStep::Step7;
                }
                JumpStep::Step7 => {
                    app.jump_step = JumpStep::Step1;
                }
            }
        }
        MenuTabs::Combat => {
            if app.combat.is_some() && !app.combat.as_ref().unwrap().scout_half {
                let mut combat = app.combat.clone().unwrap();
                enemy_turn(&mut combat, app);
                app.combat = Some(combat);
            } else if app.combat.is_some() && app.combat.as_ref().unwrap().scout_half {
                // TODO: debug only, delete this branch
                let mut combat = app.combat.clone().unwrap();
                combat.combat_text = format!(
                    "{:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                    combat.scout_turns[0],
                    combat.scout_turns[1],
                    combat.scout_turns[2],
                    combat.scout_turns[3],
                    combat.scout_turns[4],
                    combat.scout_turns[5],
                    combat.laser_fired,
                );
                app.combat = Some(combat);
            }
        }
        _ => {}
    }
}
