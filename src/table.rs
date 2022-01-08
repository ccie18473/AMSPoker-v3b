use crate::prelude::*;

const CARD_DECK_POS: usize = 0;
const CARD1_POS: usize = 1;
const CARD2_POS: usize = 2;
const CARD3_POS: usize = 3;
const CARD4_POS: usize = 4;
const CARD5_POS: usize = 5;
//const CARD_DOUBLE: usize = 6;
const BET_BTN_POS: usize = 21;
const MAX_BET_BTN_POS: usize = 22;
const RETRY_BTN_POS: usize = 23;
const CREDIT_BTN_POS: usize = 24;
const DOUBLE_BTN_POS: usize = 25;
const RED_BTN_POS: usize = 26;
const BLACK_BTN_POS: usize = 27;

pub struct TTable {
    pub game: TGame,
    pub button_set: TButtons,
    pub image: graphics::Image,
    pub card_width: f32,
    pub card_height: f32,
}

impl TTable {
    pub fn new_table(ctx: &mut Context) -> Self {
        Self {
            game: TGame::new_game(),
            button_set: TButtons::new_buttons(),
            image: graphics::Image::new(ctx, "Table.png").unwrap(),
            card_width: 0.0,
            card_height: 0.0,
        }
    }
    pub fn table_pos(&mut self, x: f32, y: f32) -> usize {
        if x < self.card_width && y < self.card_height {
            0
        } else if x > self.card_width && x < 2.0 * self.card_width && y < self.card_height {
            1
        } else if x > 2.0 * self.card_width && x < 3.0 * self.card_width && y < self.card_height {
            2
        } else if x > 3.0 * self.card_width && x < 4.0 * self.card_width && y < self.card_height {
            3
        } else if x > 4.0 * self.card_width && x < 5.0 * self.card_width && y < self.card_height {
            4
        } else if x > 5.0 * self.card_width && x < 6.0 * self.card_width && y < self.card_height {
            5
        } else if x > 6.0 * self.card_width && y < self.card_height {
            6
        } else if x < self.card_width && y > 3.0 * self.card_height {
            21
        } else if x > self.card_width && x < 2.0 * self.card_width && y > 3.0 * self.card_height {
            22
        } else if x > 2.0 * self.card_width
            && x < 3.0 * self.card_width
            && y > 3.0 * self.card_height
        {
            23
        } else if x > 3.0 * self.card_width
            && x < 4.0 * self.card_width
            && y > 3.0 * self.card_height
        {
            24
        } else if x > 4.0 * self.card_width
            && x < 5.0 * self.card_width
            && y > 3.0 * self.card_height
        {
            25
        } else if x > 5.0 * self.card_width
            && x < 6.0 * self.card_width
            && y > 3.0 * self.card_height
        {
            26
        } else if x > 6.0 * self.card_width && y > 3.0 * self.card_height {
            27
        } else {
            99
        }
    }
    pub fn process_mouse_down(&mut self, x: f32, y: f32) {
        if self.table_pos(x, y) == BET_BTN_POS {
            if self.game.eog_flag == true {
                self.game.state = 0;
                self.game.eog_flag = false;
            }
            if self.game.state == 0 || self.game.state == 1 {
                // State 0 --> State 1
                self.game.state = 1;
                if self.game.panel.bets < 10 {
                    self.button_set.buttons[0].image = FERRIS_IMG;
                    self.game.panel.credits -= 1;
                    self.game.panel.bets += 1;
                    self.game.panel.value += 1;
                }
            }
        } else if self.table_pos(x, y) == MAX_BET_BTN_POS {
            if self.game.eog_flag == true {
                self.game.state = 0;
                self.game.eog_flag = false;
            }
            if self.game.state == 0 || self.game.state == 1 {
                // State 0 --> State 1
                self.game.state = 1;
                if self.game.panel.bets < 10 {
                    self.button_set.buttons[1].image = FERRIS_IMG;
                    self.game.panel.credits = self.game.panel.credits - (10 - self.game.panel.bets);
                    self.game.panel.bets = self.game.panel.bets + (10 - self.game.panel.bets);
                    self.game.panel.value = 10;
                }
            }
        } else if self.table_pos(x, y) == RETRY_BTN_POS {
            if !self.game.eog_flag {
                if self.game.state == 2 {
                    // State 2 --> State 3
                    self.button_set.buttons[2].image = FERRIS_IMG;
                    self.game.state = 3;
                    self.game.draw_again();
                    self.game.check_prizes();
                    if self.game.win_flag {
                    } else {
                        self.game.end_of_play();
                    }
                }
            }
        } else if self.table_pos(x, y) == CREDIT_BTN_POS {
            if !self.game.eog_flag && self.game.win_flag {
                if self.game.state == 2
                    || self.game.state == 3
                    || self.game.state == 4
                    || self.game.state == 5
                {
                    // State 2, 3, 4 or 5
                    self.button_set.buttons[3].image = FERRIS_IMG;
                    self.game.panel.credits = self.game.panel.credits + self.game.panel.wins;
                    self.game.panel.msg = MSG1.to_string();
                    self.game.end_of_play();
                }
            }
        } else if self.table_pos(x, y) == DOUBLE_BTN_POS {
            if !self.game.eog_flag {
                if self.game.state == 3 {
                    // State 3 --> State 4
                    self.button_set.buttons[4].image = FERRIS_IMG;
                    self.game.state = 4;
                    self.game.panel.msg = MSG3.to_string();
                }
            }
        } else if self.table_pos(x, y) == RED_BTN_POS {
            if !self.game.eog_flag {
                // State 4 or 5
                if self.game.state == 4 || self.game.state == 5 {
                    // State 4 --> State 5
                    self.button_set.buttons[5].image = FERRIS_IMG;
                    self.game.state = 5;
                    self.game.card_double = self.game.draw_card();
                    if self.game.card_double == TCard::default() {
                        return;
                    }
                    if self.game.card_double.color() == SUIT_COLOR_RED {
                        self.game.panel.wins = 2 * self.game.panel.wins;
                        self.game.panel.msg = MSG5.to_string();
                    } else {
                        self.game.end_of_play();
                    }
                }
            }
        } else if self.table_pos(x, y) == BLACK_BTN_POS {
            if !self.game.eog_flag {
                // State 4 or 5
                if self.game.state == 4 || self.game.state == 5 {
                    // State 4 --> State 5
                    self.button_set.buttons[6].image = FERRIS_IMG;
                    self.game.state = 5;
                    self.game.card_double = self.game.draw_card();
                    if self.game.card_double == TCard::default() {
                        return;
                    }
                    if self.game.card_double.color() == SUIT_COLOR_BLACK {
                        self.game.panel.wins = 2 * self.game.panel.wins;
                        self.game.panel.msg = MSG5.to_string();
                    } else {
                        self.game.end_of_play();
                    }
                }
            }
        } else if self.table_pos(x, y) == CARD_DECK_POS {
            if !self.game.eog_flag {
                if self.game.state == 1 && self.game.panel.bets > 0 {
                    // State 1 --> State 2
                    self.game.state = 2;
                    self.game.panel.msg = MSG2.to_string();
                    self.game.hand = TDeck::new_empty_hand();
                    self.game.draw_five();
                    self.game.check_prizes();
                    self.game.deck.image = FERRIS_IMG;
                }
            }
        } else if self.table_pos(x, y) == CARD1_POS {
            if self.game.state == 2 {
                TCard::card_selected(&mut self.game.hand.cards[0]);
            }
        } else if self.table_pos(x, y) == CARD2_POS {
            if self.game.state == 2 {
                TCard::card_selected(&mut self.game.hand.cards[1]);
            }
        } else if self.table_pos(x, y) == CARD3_POS {
            if self.game.state == 2 {
                TCard::card_selected(&mut self.game.hand.cards[2]);
            }
        } else if self.table_pos(x, y) == CARD4_POS {
            if self.game.state == 2 {
                TCard::card_selected(&mut self.game.hand.cards[3]);
            }
        } else if self.table_pos(x, y) == CARD5_POS {
            if self.game.state == 2 {
                TCard::card_selected(&mut self.game.hand.cards[4]);
            }
        } else {
            //println!("Tapped Table");
        }
    }
    pub fn process_mouse_up(&mut self, x: f32, y: f32) {
        if self.table_pos(x, y) == BET_BTN_POS {
            self.button_set.buttons[0].image = BET_IMG;
        } else if self.table_pos(x, y) == MAX_BET_BTN_POS {
            self.button_set.buttons[1].image = MAX_BET_IMG;
        } else if self.table_pos(x, y) == RETRY_BTN_POS {
            self.button_set.buttons[2].image = RETRY_IMG;
        } else if self.table_pos(x, y) == CREDIT_BTN_POS {
            self.button_set.buttons[3].image = CREDIT_IMG;
        } else if self.table_pos(x, y) == DOUBLE_BTN_POS {
            self.button_set.buttons[4].image = DOUBLE_IMG;
        } else if self.table_pos(x, y) == RED_BTN_POS {
            self.button_set.buttons[5].image = RED_IMG;
        } else if self.table_pos(x, y) == BLACK_BTN_POS {
            self.button_set.buttons[6].image = BLACK_IMG;
        } else if self.table_pos(x, y) == CARD_DECK_POS {
            self.game.deck.image = RUST_IMG;
        }
    }
}
