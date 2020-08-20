use crate::game::{Dice, ShutTheBox, Statistics};
use tui::widgets::ListState;

const IDLE_TASKS: [&str; 5] = [
    "Play Manually!",
    "Autoplay: 1x",
    "Autoplay: 10x",
    "Autoplay: Fast",
    "Autoplay: Plaid",
];

const MANUAL_TASKS: [&str; 2] = ["Lock Selection", "Return"];
const LOST_TASKS: [&str; 2] = ["YOU LOST -- Retry?", "Return"];
const WON_TASKS: [&str; 2] = ["YOU WON -- Play Again?", "Return"];

#[derive(PartialEq)]
pub enum AppState {
    Idle,
    ManualGame,
    Auto1x,
    Auto10x,
    AutoFast,
    AutoPlaid,
}

// pub mod AppState {
//     use super::App;
//     pub trait Machine {
//         fn new() -> Self
//         where
//             Self: Sized;
//         fn on_up(self, app: &mut App);
//         fn on_down(self, app: &mut App);
//     }

//     pub struct Idle;
//     impl Machine for Idle {
//         fn new() -> Self {
//             Idle
//         }
//         fn on_up(self, app: &mut App) {
//             app.tasks.previous();
//         };
//         fn on_down(self, app: &mut App) {
//             app.tasks.next();
//         };
//     }
// }

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub state: AppState,
    pub tasks: StatefulList<&'a str>,
    pub game: ShutTheBox,
    pub stats: Statistics,
    pub selection: usize,
    pub dice: Dice,
    pub staging: Vec<usize>,
    pub gameover: bool,
    pub plotidx: usize,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            should_quit: false,
            state: AppState::Idle,
            tasks: StatefulList::with_items(IDLE_TASKS.to_vec()),
            game: ShutTheBox::init(12),
            stats: Statistics::new(),
            selection: 0,
            dice: Dice::new(),
            staging: Vec::with_capacity(5),
            gameover: false,
            plotidx: 0,
        }
    }

    pub fn on_up(&mut self) {
        match self.state {
            AppState::Idle => self.tasks.previous(),
            AppState::ManualGame => match self.tasks.state.selected() {
                Some(0) => {
                    if !self.game.victory() {
                        self.tasks.state.select(None);
                    }
                }
                Some(_) => {
                    self.tasks.previous();
                }
                None => {}
            },
            _ => {}
        }
    }

    pub fn on_down(&mut self) {
        match self.state {
            AppState::Idle => self.tasks.next(),
            AppState::ManualGame => match self.tasks.state.selected() {
                Some(idx) => {
                    if idx == self.tasks.items.len() - 1 {
                        if !self.game.victory() {
                            self.tasks.state.select(None);
                        }
                    } else {
                        self.tasks.next();
                    }
                }
                None => {
                    self.tasks.state.select(Some(0));
                }
            },
            _ => {}
        }
    }

    fn return_to_menu(&mut self) {
        self.gameover = false;
        self.state = AppState::Idle;
        self.tasks = StatefulList::with_items(IDLE_TASKS.to_vec());
        self.tasks.state.select(Some(0));
        self.game = ShutTheBox::init(12);
    }

    fn end_game(&mut self, result: bool) {
        let items = if result { WON_TASKS } else { LOST_TASKS };
        self.gameover = true;
        self.tasks = StatefulList::with_items(items.to_vec());
        self.tasks.state.select(Some(0));
        self.stats.save_game(&self.game);
    }

    fn reroll(&mut self) {
        self.tasks.state.select(None);
        self.staging.clear();
        let open = self.game.get_open();
        if open.len() == 0 {
            // CONGRATULATIONS! You win!
            self.end_game(true);
            return;
        }
        if open.len() < self.game.total {
            self.selection = *open
                .iter()
                .skip_while(|&x| *x <= self.selection)
                .next()
                .unwrap_or(&(&open[0] + 1))
                - 1;
        }
        self.dice.roll();
        self.game.save_roll(self.dice.result());
        if self.game.check_loss(self.dice.result()) {
            self.end_game(false);
            self.game.check_loss(self.dice.result());
        }
    }

    fn new_game(&mut self) {
        self.state = AppState::ManualGame;
        self.tasks = StatefulList::with_items(MANUAL_TASKS.to_vec());
        self.gameover = false;
        self.game = ShutTheBox::init(12);
        self.selection = 0;
        self.reroll();
    }

    pub fn on_enter(&mut self) {
        match self.state {
            AppState::Idle => {
                // Start selected game
                match self.tasks.state.selected() {
                    Some(0) => self.new_game(),
                    _ => {}
                }
            }
            AppState::ManualGame => {
                // Check if tasks are selected
                match self.tasks.state.selected() {
                    None => {
                        // BOXES Selected
                        // If box selection is in staging vector, remove it
                        match self.staging.iter().position(|&x| x == self.selection) {
                            Some(idx) => {
                                self.staging.remove(idx);
                            }
                            None => {
                                self.staging.push(self.selection);
                            }
                        }
                    }
                    Some(0) => {
                        // First Item in List
                        if self.gameover {
                            self.new_game();
                        } else {
                            if self.staging.iter().fold(0, |acc, x| acc + x + 1)
                                == self.dice.result()
                            {
                                // Lock! Shut the boxes
                                for val in self.staging.iter() {
                                    self.game.shut(*val + 1);
                                }
                                self.reroll();
                            }
                        }
                    }
                    Some(1) => {
                        // Return to main menu!
                        self.return_to_menu();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn on_right(&mut self) {
        match self.state {
            AppState::ManualGame => {
                if self.tasks.state.selected().is_some() {
                    // Dont move left/right when boxes are selected
                    return;
                }
                self.select_next();
                while self.game.get_status(self.selection + 1).unwrap() {
                    self.select_next();
                }
            }
            _ => {}
        }
    }

    pub fn on_left(&mut self) {
        match self.state {
            AppState::ManualGame => {
                if self.tasks.state.selected().is_some() {
                    // Dont move left/right when boxes are selected
                    return;
                }
                self.select_prev();
                while self.game.get_status(self.selection + 1).unwrap() {
                    self.select_prev();
                }
            }
            _ => {}
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'p' => {
                self.plotidx += 1;
                if self.plotidx > 3 {
                    self.plotidx = 0;
                }
            }
            '\n' => {
                self.on_enter();
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
        // self.progress += 0.001;
        // if self.progress > 1.0 {
        //     self.progress = 0.0;
        // }

        // self.sparkline.on_tick();
        // self.signals.on_tick();

        // let log = self.logs.items.pop().unwrap();
        // self.logs.items.insert(0, log);

        // let event = self.barchart.pop().unwrap();
        // self.barchart.insert(0, event);
    }

    fn select_next(&mut self) {
        self.selection += 1;
        if self.selection == self.game.total {
            self.selection = 0;
        };
    }
    fn select_prev(&mut self) {
        if self.selection == 0 {
            self.selection = self.game.total;
        };
        self.selection -= 1;
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
