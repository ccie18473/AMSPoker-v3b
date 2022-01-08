use crate::prelude::*;

// Panel
pub const MSG1: &str = "Bet then tap the Deck Card";
pub const MSG2: &str = "Select those you don't want";
pub const MSG3: &str = "Good Luck";
pub const MSG4: &str = "You won, select or credit";
pub const MSG5: &str = "You won, double or credit";
pub const MSG6: &str = "No more cards in the Deck";
pub struct TPanel {
    pub prize: String,
    pub value: usize,
    pub bets: isize,
    pub wins: isize,
    pub msg: String,
    pub credits: isize,
}

impl TPanel {
    pub fn new_panel() -> Self {
        Self {
            prize: "".to_string(),
            value: 0,
            bets: 0,
            wins: 0,
            msg: MSG1.to_string(),
            credits: CREDITS,
        }
    }
}
