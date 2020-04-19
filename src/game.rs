use rand::{thread_rng, seq::SliceRandom};
use std::{str::FromStr, fmt};

pub enum Outcome {
    Win,
    Lose,
    Draw,
}

pub enum Item {
    Rock,
    Paper,
    Scissors,
}

impl Item {
    const ITEMS: [Self;3] = [Self::Rock, Self::Paper, Self::Scissors];

    pub fn random() -> &'static Self {
        Self::ITEMS
            .choose(&mut thread_rng())
            .unwrap()
    }

    pub fn versus(&self, other: &Self) -> Outcome {
        use Item::*;
        use Outcome::*;

        match (self, other) {
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,

            (Paper, Scissors) => Lose,
            (Paper, Rock) => Win,

            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,

            _ => Draw,
        }
    }
}

impl FromStr for Item {
    type Err = ();

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        use Item::*;

        let item = match source.trim().to_lowercase().as_str() {
            "rock" => Rock,
            "paper" => Paper,
            "scissors" => Scissors,
            _ => return Err(()),
        };

        Ok(item)
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Item::*;

        write!(f, "{}", match self {
            Rock => "🗿",
            Paper => "📄",
            Scissors => "✂",
        })
    }
}
