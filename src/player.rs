#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    X,
    O,
    None
}

impl Player {
    pub fn from(s: &str) -> Player {
        match s {
            "X" => Player::X,
            "O" => Player::O,
            _ => Player::None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Player::X => String::from("X"),
            Player::O => String::from("O"),
            _ => String::from(" ")
        }
    }
}