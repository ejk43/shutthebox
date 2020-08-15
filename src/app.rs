use crate::game::{ShutTheBox, Statistics};
use tui::widgets::ListState;

const TASKS: [&str; 5] = [
    "Play Manually!",
    "Autoplay: 1x",
    "Autoplay: 10x",
    "Autoplay: Fast",
    "Autoplay: Plaid",
];

pub enum AppState {
    Idle,
    ManualGame,
    Auto1x,
    Auto10x,
    AutoFast,
    AutoPlaid,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub state: AppState,
    pub tasks: StatefulList<&'a str>,
    pub game: ShutTheBox,
    pub stats: Statistics,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            should_quit: false,
            state: AppState::Idle,
            tasks: StatefulList::with_items(TASKS.to_vec()),
            game: ShutTheBox::init(12),
            stats: Statistics::new(),
        }
    }

    pub fn on_up(&mut self) {
        self.tasks.previous();
    }

    pub fn on_down(&mut self) {
        self.tasks.next();
    }

    pub fn on_enter(&mut self) {
        match self.state {
            AppState::Idle => {
                // Start selected game
                match self.tasks.state.selected() {
                    Some(0) => {
                        self.state = AppState::ManualGame;
                        self.game = ShutTheBox::init(12);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    // pub fn on_right(&mut self) {
    //     self.tabs.next();
    // }

    // pub fn on_left(&mut self) {
    //     self.tabs.previous();
    // }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
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
