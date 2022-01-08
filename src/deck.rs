use crate::prelude::*;

type Suit = usize;
type SuitColor = usize;

pub const SUIT_CLUBS: Suit = 0;
//pub const SUIT_DIAMONDS: Suit = 1;
//pub const SUIT_HEARTS: Suit = 2;
pub const SUIT_SPADES: Suit = 3;

pub const SUIT_COLOR_BLACK: SuitColor = 0;
pub const SUIT_COLOR_RED: SuitColor = 1;

//pub const VALUE_JACK: usize = 10;
//pub const VALUE_QUEEN: usize = 11;
pub const VALUE_KING: usize = 12;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct TCard {
    pub value: usize,
    pub suit: Suit,
    pub face_up: bool,
    pub selected: bool,
    pub image: usize,
}

impl TCard {
    pub fn new_card(value: usize, suit: Suit, face_up: bool, selected: bool, image: usize) -> Self {
        Self {
            value,
            suit,
            face_up,
            selected,
            image,
        }
    }
    pub fn card_selected(&mut self) {
        self.flip_card()
    }
    pub fn flip_card(&mut self) {
        self.face_up = !self.face_up;
        self.selected = !self.selected;
        if self.selected {
            self.image = VALUE_RUST;
        } else {
            self.image = self.value + self.suit * 13;
        }
    }
    pub fn color(&self) -> usize {
        if self.suit == SUIT_CLUBS || self.suit == SUIT_SPADES {
            return SUIT_COLOR_BLACK;
        }
        SUIT_COLOR_RED
    }
}

pub struct TDeck {
    pub cards: Vec<TCard>,
    pub image: usize,
}
impl TDeck {
    pub fn new_sorted_deck() -> Self {
        let mut deck = TDeck {
            cards: Vec::new(),
            image: RUST_IMG,
        };
        let mut index = 0;
        for suit in 0..SUIT_SPADES + 1 {
            for value in 0..VALUE_KING + 1 {
                deck.cards
                    .push(TCard::new_card(value, suit, false, false, index));
                index += 1;
            }
        }
        deck
    }
    pub fn new_empty_hand() -> Self {
        let hand = TDeck {
            cards: Vec::new(),
            image: RUST_IMG,
        };
        hand
    }
    pub fn shuffle(&mut self) {
        qrand::srand(miniquad::date::now() as _);
        for n in 0..self.cards.len() {
            let m = qrand::gen_range::<usize>(0, self.cards.len());
            if m != n {
                self.cards.swap(m, n);
            }
        }
    }
    pub fn new_shuffled_deck() -> Self {
        let mut deck = TDeck::new_sorted_deck();
        TDeck::shuffle(&mut deck);
        deck
    }
    pub fn pop(&mut self) -> Option<TCard> {
        self.cards.pop()
    }
}
