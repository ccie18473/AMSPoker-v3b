pub const RUST_IMG: usize = 52;
pub const BET_IMG: usize = 53;
pub const MAX_BET_IMG: usize = 54;
pub const RETRY_IMG: usize = 55;
pub const CREDIT_IMG: usize = 56;
pub const DOUBLE_IMG: usize = 57;
pub const RED_IMG: usize = 58;
pub const BLACK_IMG: usize = 59;
pub const FERRIS_IMG: usize = 60;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct TButton {
    pub image: usize,
}

impl TButton {
    pub fn new(value: usize) -> Self {
        Self { image: value }
    }
}

pub struct TButtons {
    pub buttons: Vec<TButton>,
}

impl TButtons {
    pub fn new_buttons() -> Self {
        let mut button_set = TButtons {
            buttons: Vec::new(),
        };

        let bet_btn = TButton::new(BET_IMG);
        button_set.buttons.push(bet_btn);

        let max_bet_btn = TButton::new(MAX_BET_IMG);
        button_set.buttons.push(max_bet_btn);

        let retry_btn = TButton::new(RETRY_IMG);
        button_set.buttons.push(retry_btn);

        let credit_btn = TButton::new(CREDIT_IMG);
        button_set.buttons.push(credit_btn);

        let double_btn = TButton::new(DOUBLE_IMG);
        button_set.buttons.push(double_btn);

        let red_btn = TButton::new(RED_IMG);
        button_set.buttons.push(red_btn);

        let black_btn = TButton::new(BLACK_IMG);
        button_set.buttons.push(black_btn);

        button_set
    }
}
