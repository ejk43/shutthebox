use hdrhistogram::Histogram;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn simulate_game() -> ShutTheBox {
    let mut game = ShutTheBox::init(12);
    let mut dice = Dice::new();
    let mut valid = true;
    while valid && !game.victory() {
        dice.roll();
        valid = game.play_roll(dice.result());
    }
    game
}

#[derive(Debug)]
pub struct Statistics {
    num_won: u64,
    num_total: u64,
    last_won: u64,
    pub games_between_win: Histogram<u64>,
    count_shut: Vec<u64>,
    count_nrolls: Vec<u64>,
    count_lastroll: Vec<u64>,
}

impl Statistics {
    pub fn new() -> Statistics {
        let mut default_hash = HashMap::with_capacity(12);
        for ii in 0..12 {
            default_hash.insert(ii, 0.0);
        }
        Statistics {
            num_won: 0,
            num_total: 0,
            last_won: 0,
            games_between_win: Histogram::<u64>::new(4).unwrap(),
            count_shut: vec![0; 12],
            count_nrolls: vec![0; 12],
            count_lastroll: vec![0; 12],
        }
    }

    pub fn save_game(&mut self, game: &ShutTheBox) {
        self.num_won += game.victory() as u64;
        if game.victory() {
            self.games_between_win
                .record((self.num_total - self.last_won) as u64)
                .expect("Could not add value to histogram");
            self.last_won = self.num_total;
        }
        self.num_total += 1;
        for (ii, shut) in game.status.iter().enumerate() {
            self.count_shut[ii] += *shut as u64;
        }
        // Save Rolls
        let rolls = game.get_rolls();
        let nrolls = rolls.len();
        self.count_nrolls[nrolls - 1] += 1;
        self.count_lastroll[rolls[nrolls - 1] - 1] += 1;
    }
}

struct VecTotal {
    values: Vec<usize>,
    total: usize,
}

impl VecTotal {
    fn push(&mut self, val: usize) {
        self.total += val;
        self.values.push(val)
    }
    fn pop(&mut self) -> usize {
        match self.values.pop() {
            Some(val) => {
                self.total -= val;
                val
            }
            None => 0,
        }
    }
}

fn check_slice(sumvec: &mut VecTotal, slice: &[usize], target: usize) -> bool {
    if slice.len() == 0 {
        return false;
    }
    for ii in 0..slice.len() {
        sumvec.push(slice[ii]);
        let hit = match sumvec.total.cmp(&target) {
            Ordering::Less => check_slice(sumvec, &slice[ii + 1..], target),
            Ordering::Greater => {
                sumvec.pop();
                check_slice(sumvec, &slice[ii + 1..], target)
            }
            Ordering::Equal => true,
        };
        if hit {
            return true;
        }
        sumvec.pop();
    }
    false
}

/// ShutTheBox struct represents the game board
#[derive(Debug)]
pub struct ShutTheBox {
    status: Vec<bool>, // Indicate TRUE if SHUT
    rolls: Vec<usize>, // Ordered Roll History
    shut: Vec<usize>,  // Ordered Number History
    pub total: usize,  // Number of boxes
}

impl ShutTheBox {
    /// Initialize ShutTheBox with a max size
    pub fn init(total: usize) -> ShutTheBox {
        ShutTheBox {
            status: vec![false; total],
            rolls: Vec::with_capacity(total),
            shut: Vec::with_capacity(total),
            total: total,
        }
    }

    pub fn get_status(&self, val: usize) -> Option<bool> {
        // 1-indexed... cannot be 0 or > capacity
        if val == 0 || val > self.status.len() {
            return None;
        }
        Some(self.status[val - 1])
    }

    pub fn shut(&mut self, val: usize) {
        if val > 0 && val < self.status.len() + 1 {
            self.shut.push(val);
            self.status[val - 1] = true;
        }
    }

    /// Check for victory
    pub fn victory(&self) -> bool {
        self.status.iter().all(|&x| x)
    }

    /// Check for loss, given a particular value and game state
    pub fn check_loss(&self, target: usize) -> bool {
        let open: Vec<usize> = self.iter_open().filter(|x| *x <= target).collect();
        let mut sumvec = VecTotal {
            values: Vec::new(),
            total: 0,
        };
        let alive = check_slice(&mut sumvec, &open[0..], target);
        // if alive {
        //     println!("Valid Open: {:?}", sumvec.values);
        // }
        !alive
    }

    /// Attempt to play a roll
    /// Returns true if the roll was played successfully...
    /// Returns false if the game is OVER
    pub fn play_roll(&mut self, roll: usize) -> bool {
        // Save the roll
        self.rolls.push(roll);

        // Check if open
        let open = match self.get_status(roll) {
            Some(false) => true,
            Some(true) => false,
            None => false,
        };

        // Play the roll if its open
        if open {
            self.shut(roll);
            return true;
        } else {
            // Try to split the roll if otherwise
            for ii in 1..(roll as f32 / 2.0).ceil() as usize {
                let closed_high = self.get_status(roll - ii).unwrap();
                let closed_low = self.get_status(ii).unwrap();
                if !closed_low && !closed_high {
                    self.shut(roll - ii);
                    self.shut(ii);
                    return true;
                }
            }
            // Next try to split the roll N-way
            let open: Vec<usize> = self.iter_open().filter(|x| *x <= roll).collect();
            let mut sumvec = VecTotal {
                values: Vec::new(),
                total: 0,
            };
            let alive = check_slice(&mut sumvec, &open[0..], roll);
            if alive {
                for ii in sumvec.values.iter() {
                    self.shut(*ii);
                }
                return true;
            }
            return false;
        }
    }

    /// Return ordered vector of numbers that have been shut
    pub fn get_shut(&self) -> Vec<usize> {
        // Ordered list of numbers that have been shut
        // (1..self.status.len() + 1)
        //     .filter(|ii| self.get(*ii).unwrap())
        //     .collect()
        self.iter_shut().collect()
    }

    pub fn iter_shut<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        // Iterator of numbers that have been shut
        // (1..self.status.len() + 1)
        //     .filter(|ii| self.get(*ii).unwrap())
        //     .collect()
        self.status
            .iter()
            .enumerate()
            .filter(|shut| *shut.1)
            .map(|pair| pair.0 + 1)
    }

    /// Return ordered vector of numbers that are still open
    pub fn get_open(&self) -> Vec<usize> {
        self.iter_open().collect()
    }

    pub fn iter_open<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        // Iterator of numbers that are still open
        self.status
            .iter()
            .enumerate()
            .filter(|shut| !*shut.1)
            .map(|pair| pair.0 + 1)
    }

    /// Return vector of rolls, in the order they were played
    pub fn get_rolls<'a>(&'a self) -> &'a Vec<usize> {
        &self.rolls
    }

    // Return vector of numbers, in the order they were played
    pub fn get_numbers<'a>(&'a self) -> &'a Vec<usize> {
        &self.shut
    }
}

/// Struct for handling a 2-Dice roll
pub struct Dice {
    rng: rand::rngs::ThreadRng,
    pub values: (usize, usize),
}

impl Dice {
    pub fn new() -> Dice {
        Dice {
            rng: rand::thread_rng(),
            values: (0, 0),
        }
    }

    pub fn roll(&mut self) {
        self.values = (self.rng.gen_range(1, 7), self.rng.gen_range(1, 7));
    }

    pub fn result(&self) -> usize {
        self.values.0 + self.values.1
    }

    pub fn pprint(&self) -> String {
        fn onedie(val: usize) -> &'static str {
            match val {
                1 => "⚀",
                2 => "⚁",
                3 => "⚂",
                4 => "⚃",
                5 => "⚄",
                6 => "⚅",
                _ => "?",
            }
        }
        format!("{} {}", onedie(self.values.0), onedie(self.values.1))
    }
}

#[cfg(test)]
mod tests {
    use super::Dice;
    use super::ShutTheBox;

    #[test]
    fn test_dice_roll() {
        let mut dice = Dice::new();
        dice.roll();
        assert_eq!(dice.result(), dice.values.0 + dice.values.1);
    }

    #[test]
    fn test_shutthebox_loss() {
        let max = 12;
        let mut game = ShutTheBox::init(max);
        assert_eq!(game.check_loss(10), false);
        game.shut(10);
        assert_eq!(game.check_loss(10), false);
        game.shut(9);
        assert_eq!(game.check_loss(10), false);
        game.shut(8);
        assert_eq!(game.check_loss(10), false);
        game.shut(7);
        assert_eq!(game.check_loss(10), false);
        game.shut(6);
        assert_eq!(game.check_loss(10), false);
        game.shut(4);
        assert_eq!(game.check_loss(10), false);
        game.shut(1);
        assert_eq!(game.check_loss(10), false);
        game.shut(2);
        assert_eq!(game.check_loss(10), true);
    }

    #[test]
    fn test_shutthebox_loss_edge() {
        let max = 12;
        let mut game = ShutTheBox::init(max);
        assert_eq!(game.check_loss(10), false);
        game.play_roll(5);
        game.play_roll(9);
        game.play_roll(7);
        game.play_roll(11);
        game.play_roll(10);
        game.play_roll(6);
        game.play_roll(11);
        assert_eq!(game.check_loss(11), true);
    }

    #[test]
    fn test_shutthebox_loss_edge2() {
        let max = 12;
        let mut game = ShutTheBox::init(max);
        assert_eq!(game.check_loss(12), false);
        game.shut(12);
        game.shut(11);
        game.shut(10);
        game.shut(9);
        game.shut(8);
        game.shut(7);
        game.shut(3);
        assert_eq!(game.check_loss(12), false);
        // game.shut(5);
        assert_eq!(game.check_loss(11), false);
    }

    #[test]
    fn test_shutthebox_play_3() {
        let max = 4;
        let mut game = ShutTheBox::init(max);
        let valid = game.play_roll(3);
        assert_eq!(valid, true);
        assert_eq!(game.get_status(3).unwrap(), true);
        let valid = game.play_roll(3);
        assert_eq!(valid, true);
        assert_eq!(game.get_status(1).unwrap(), true);
        assert_eq!(game.get_status(2).unwrap(), true);
        assert_eq!(game.get_status(3).unwrap(), true);
    }

    #[test]
    fn test_shutthebox_play_4() {
        let max = 9;
        let mut game = ShutTheBox::init(max);
        let valid = game.play_roll(4);
        assert_eq!(valid, true);
        assert_eq!(game.get_status(4).unwrap(), true);
        let valid = game.play_roll(4);
        assert_eq!(valid, true);
        assert_eq!(game.get_status(1).unwrap(), true);
        assert_eq!(game.get_status(3).unwrap(), true);
        assert_eq!(game.get_status(2).unwrap(), false);
        let valid = game.play_roll(4);
        assert_eq!(valid, false);
    }

    #[test]
    fn test_shutthebox_play_5() {
        let max = 5;
        let mut game = ShutTheBox::init(max);
        let valid = game.play_roll(5);
        assert_eq!(valid, true);
        assert_eq!(game.get_status(5).unwrap(), true);
        let valid = game.play_roll(5);
        assert_eq!(valid, true);
        assert_eq!(game.get_status(1).unwrap(), true);
        assert_eq!(game.get_status(4).unwrap(), true);
        let valid = game.play_roll(5);
        assert_eq!(valid, true);
        assert_eq!(game.get_status(2).unwrap(), true);
        assert_eq!(game.get_status(3).unwrap(), true);
    }

    #[test]
    fn test_shutthebox_play_6() {
        let max = 9;
        let mut game = ShutTheBox::init(max);
        let valid = game.play_roll(6);
        assert_eq!(valid, true);
        let valid = game.play_roll(6);
        assert_eq!(valid, true);
        let valid = game.play_roll(6);
        assert_eq!(valid, true);
        assert_eq!(game.get_status(1).unwrap(), true);
        assert_eq!(game.get_status(2).unwrap(), true);
        assert_eq!(game.get_status(3).unwrap(), false);
        assert_eq!(game.get_status(4).unwrap(), true);
        assert_eq!(game.get_status(5).unwrap(), true);
        assert_eq!(game.get_status(6).unwrap(), true);
        let valid = game.play_roll(6);
        assert_eq!(valid, false);
    }

    #[test]
    fn test_shutthebox_init_9() {
        let max = 9;
        let game = ShutTheBox::init(max);
        assert_eq!(game.status.capacity(), 9);
        for ii in 1..max + 1 {
            assert_eq!(game.get_status(ii), Some(false));
        }
    }

    #[test]
    fn test_shutthebox_init_12() {
        let max = 12;
        let game = ShutTheBox::init(max);
        assert_eq!(game.status.capacity(), 12);
        for ii in 1..max + 1 {
            assert_eq!(game.get_status(ii), Some(false));
        }
    }

    #[test]
    fn test_shutthebox_outofbound() {
        let max = 9;
        let game = ShutTheBox::init(max);
        assert_eq!(game.status.capacity(), 9);
        assert_eq!(game.get_status(9), Some(false));
        assert_eq!(game.get_status(0), None);
        assert_eq!(game.get_status(10), None);
    }

    #[test]
    fn test_shutthebox_shut_1() {
        let mut game = ShutTheBox::init(12);
        game.shut(1);
        assert_eq!(game.get_status(1), Some(true));
    }

    #[test]
    fn test_shutthebox_victory() {
        let mut game = ShutTheBox::init(3);
        assert_eq!(game.victory(), false);
        game.shut(1);
        assert_eq!(game.victory(), false);
        game.shut(2);
        assert_eq!(game.victory(), false);
        game.shut(3);
        assert_eq!(game.victory(), true);
    }

    #[test]
    fn test_shutthebox_get_shut_and_open() {
        let mut game = ShutTheBox::init(4);
        game.shut(1);
        assert_eq!(game.get_shut(), vec![1]);
        assert_eq!(game.get_open(), vec![2, 3, 4]);
        game.shut(2);
        assert_eq!(game.get_shut(), vec![1, 2]);
        assert_eq!(game.get_open(), vec![3, 4]);
    }

    #[test]
    fn test_shutthebox_get_rolls_and_numbers() {
        let mut game = ShutTheBox::init(4);
        game.play_roll(4);
        assert_eq!(*game.get_rolls(), vec![4]);
        game.play_roll(4);
        assert_eq!(*game.get_rolls(), vec![4, 4]);
        assert_eq!(*game.get_numbers(), vec![4, 3, 1]);
    }
}
