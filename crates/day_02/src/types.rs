#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Draw {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reveal {
    pub draws: Vec<Draw>,
}

impl Reveal {
    pub fn red(&self) -> u32 {
        self.draws
            .iter()
            .filter_map(|draw| match draw {
                Draw::Red(count) => Some(*count),
                _ => None,
            })
            .sum()
    }

    pub fn green(&self) -> u32 {
        self.draws
            .iter()
            .filter_map(|draw| match draw {
                Draw::Green(count) => Some(*count),
                _ => None,
            })
            .sum()
    }

    pub fn blue(&self) -> u32 {
        self.draws
            .iter()
            .filter_map(|draw| match draw {
                Draw::Blue(count) => Some(*count),
                _ => None,
            })
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub id: u32,
    pub reveals: Vec<Reveal>,
}
