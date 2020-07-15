use rand::Rng;

/// ShutTheBox struct represents the game board
#[derive(Debug)]
pub struct ShutTheBox {
    fields: Vec<bool>, // Indicate TRUE if SHUT
}

impl ShutTheBox {
    /// Initialize ShutTheBox with a max size
    pub fn init(max: usize) -> ShutTheBox {
        ShutTheBox {
            fields: vec![false; max],
        }
    }

    pub fn get(&self, val: usize) -> Option<bool> {
        // 1-indexed... cannot be 0 or > capacity
        if val == 0 || val > self.fields.len() {
            return None;
        }
        Some(self.fields[val - 1])
    }

    pub fn shut(&mut self, val: usize) {
        if val > 0 && val < self.fields.len() + 1 {
            self.fields[val - 1] = true;
        }
    }

    pub fn victory(&self) -> bool {
        self.fields.iter().all(|&x| x)
    }

    pub fn play_value(&mut self, val: usize) -> bool {
        let open = match self.get(val) {
            Some(false) => true,
            Some(true) => false,
            None => false,
        };
        if open {
            self.shut(val);
            return true;
        } else {
            for ii in 1..(val / 2) {
                let closed_low = self.get(ii).unwrap();
                let closed_high = self.get(val - ii).unwrap();
                if !closed_low && !closed_high {
                    self.shut(ii);
                    self.shut(val - ii);
                    return true;
                }
            }
            return false;
        }
    }

    // pub fn get_boxes(&self) -> Vec[usize] {
    //     (1..self.fields.len()+1).self.fields.iter().all(|&x| x)
    // }
}

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
    fn test_shutthebox_play_4() {
        let max = 9;
        let mut game = ShutTheBox::init(max);
        let valid = game.play_value(4);
        assert_eq!(valid, true);
        assert_eq!(game.get(4).unwrap(), true);
        let valid = game.play_value(4);
        assert_eq!(valid, true);
        assert_eq!(game.get(1).unwrap(), true);
        assert_eq!(game.get(3).unwrap(), true);
        assert_eq!(game.get(2).unwrap(), false);
        let valid = game.play_value(4);
        assert_eq!(valid, false);
    }

    #[test]
    fn test_shutthebox_play_6() {
        let max = 9;
        let mut game = ShutTheBox::init(max);
        let valid = game.play_value(6);
        assert_eq!(valid, true);
        let valid = game.play_value(6);
        assert_eq!(valid, true);
        let valid = game.play_value(6);
        assert_eq!(valid, true);
        assert_eq!(game.get(1).unwrap(), true);
        assert_eq!(game.get(2).unwrap(), true);
        assert_eq!(game.get(3).unwrap(), false);
        assert_eq!(game.get(4).unwrap(), true);
        assert_eq!(game.get(5).unwrap(), true);
        assert_eq!(game.get(6).unwrap(), true);
        let valid = game.play_value(6);
        assert_eq!(valid, false);
    }

    #[test]
    fn test_shutthebox_init_9() {
        let max = 9;
        let game = ShutTheBox::init(max);
        assert_eq!(game.fields.capacity(), 9);
        for ii in 1..max + 1 {
            assert_eq!(game.get(ii), Some(false));
        }
    }

    #[test]
    fn test_shutthebox_outofbound() {
        let max = 9;
        let game = ShutTheBox::init(max);
        assert_eq!(game.fields.capacity(), 9);
        assert_eq!(game.get(9), Some(false));
        assert_eq!(game.get(0), None);
        assert_eq!(game.get(10), None);
    }

    #[test]
    fn test_shutthebox_init_12() {
        let max = 12;
        let game = ShutTheBox::init(max);
        assert_eq!(game.fields.capacity(), 12);
        for ii in 1..max + 1 {
            assert_eq!(game.get(ii), Some(false));
        }
    }

    #[test]
    fn test_shutthebox_shut_1() {
        let mut game = ShutTheBox::init(12);
        game.shut(1);
        assert_eq!(game.get(1), Some(true));
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
}
