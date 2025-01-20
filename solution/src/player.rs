#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn chars(&self) -> Vec<char> {
        match self {
            Player::One => vec!['@', 'a'],
            Player::Two => vec!['$', 's'],
        }
    }

    pub fn opponent_chars(&self) -> Vec<char> {
        match self {
            Player::One => Player::Two.chars(),
            Player::Two => Player::One.chars(),
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '1' => Some(Player::One),
            '2' => Some(Player::Two),
            _ => None,
        }
    }
}
