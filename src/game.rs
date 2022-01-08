use crate::prelude::*;

pub const CREDITS: isize = 100;

pub struct TGame {
    pub deck: TDeck,
    pub card_double: TCard,
    pub hand: TDeck,
    pub panel: TPanel,
    pub state: usize,
    pub eog_flag: bool,
    pub win_flag: bool,
}

impl TGame {
    pub fn new_game() -> Self {
        Self {
            deck: TDeck::new_shuffled_deck(),
            card_double: TCard::default(),
            hand: TDeck::new_empty_hand(),
            panel: TPanel::new_panel(),
            state: 0,
            eog_flag: false,
            win_flag: false,
        }
    }
    pub fn draw_card(&mut self) -> TCard {
        if self.deck.cards.len() == 0 {
            self.panel.msg = MSG6.to_string();
            return TCard::default();
        }

        let popped = self.deck.pop();

        match popped {
            Some(mut card) => {
                card.face_up = true;
                card
            }
            None => TCard::default(),
        }
    }
    pub fn remove_card(&mut self, i: usize) {
        self.hand.cards.remove(i);
    }
    pub fn insert_card(&mut self, i: usize, card: TCard) {
        self.hand.cards.insert(i, card);
    }
    pub fn draw_five(&mut self) {
        self.deck = TDeck::new_shuffled_deck();

        for i in 0..5 {
            let card = self.draw_card();
            self.insert_card(i, card);
        }

        self.card_double = TCard::new_card(99, 3, false, false, 0);
    }
    pub fn draw_again(&mut self) {
        for i in 0..5 {
            if self.hand.cards[i].selected {
                self.remove_card(i);
                let card = self.draw_card();
                self.insert_card(i, card);
            }
        }
    }
    pub fn check_prizes(&mut self) -> bool {
        let mut nmbr: [usize; 52] = [0; 52];
        for card in self.hand.cards.iter() {
            nmbr[card.image] = 1
        }
        if crate::game::royal_flush(nmbr) {
            self.panel.wins = self.panel.bets * 500;
            self.win_flag = true;
            self.panel.prize = "Royal Flush !!!".to_string();
            if self.state == 2 {
                self.panel.msg = MSG4.to_string();
            } else if self.state == 3 {
                self.panel.msg = MSG5.to_string();
            }
            return self.win_flag;
        } else if crate::game::straight_flush(nmbr) {
            self.panel.wins = self.panel.bets * 80;
            self.win_flag = true;
            self.panel.prize = "Straight Flush !!!".to_string();
            if self.state == 2 {
                self.panel.msg = MSG4.to_string();
            } else if self.state == 3 {
                self.panel.msg = MSG5.to_string();
            }
            return self.win_flag;
        } else if crate::game::four_of_a_kind(nmbr) {
            self.panel.wins = self.panel.bets * 25;
            self.win_flag = true;
            self.panel.prize = "Four of a Kind !!!".to_string();
            if self.state == 2 {
                self.panel.msg = MSG4.to_string();
            } else if self.state == 3 {
                self.panel.msg = MSG5.to_string();
            }
            return self.win_flag;
        } else if crate::game::full_house(nmbr) {
            self.panel.wins = self.panel.bets * 10;
            self.win_flag = true;
            self.panel.prize = "Full House !!!".to_string();
            if self.state == 2 {
                self.panel.msg = MSG4.to_string();
            } else if self.state == 3 {
                self.panel.msg = MSG5.to_string();
            }
            return self.win_flag;
        } else if crate::game::flush(nmbr) {
            self.panel.wins = self.panel.bets * 8;
            self.win_flag = true;
            self.panel.prize = "Flush !!!".to_string();
            if self.state == 2 {
                self.panel.msg = MSG4.to_string();
            } else if self.state == 3 {
                self.panel.msg = MSG5.to_string();
            }
            return self.win_flag;
        } else if crate::game::straight(nmbr) {
            self.panel.wins = self.panel.bets * 5;
            self.win_flag = true;
            self.panel.prize = "Straight !!!".to_string();
            if self.state == 2 {
                self.panel.msg = MSG4.to_string();
            } else if self.state == 3 {
                self.panel.msg = MSG5.to_string();
            }
            return self.win_flag;
        } else if crate::game::three_of_a_kind(nmbr) {
            self.panel.wins = self.panel.bets * 3;
            self.win_flag = true;
            self.panel.prize = "Tree of a Kind !!!".to_string();
            if self.state == 2 {
                self.panel.msg = MSG4.to_string();
            } else if self.state == 3 {
                self.panel.msg = MSG5.to_string();
            }
            return self.win_flag;
        } else if crate::game::two_pair(nmbr) {
            self.panel.wins = self.panel.bets * 2;
            self.win_flag = true;
            self.panel.prize = "Two Pair !!!".to_string();
            if self.state == 2 {
                self.panel.msg = MSG4.to_string();
            } else if self.state == 3 {
                self.panel.msg = MSG5.to_string();
            }
            return self.win_flag;
        } else if crate::game::pair_of_aces(nmbr) {
            self.panel.wins = self.panel.bets;
            self.win_flag = true;
            self.panel.prize = "Pair of Aces !!!".to_string();
            if self.state == 2 {
                self.panel.msg = MSG4.to_string();
            } else if self.state == 3 {
                self.panel.msg = MSG5.to_string();
            }
            return self.win_flag;
        } else {
            self.panel.prize = "No prize...".to_string();
            self.win_flag = false;
        }
        return self.win_flag;
    }
    pub fn end_of_play(&mut self) {
        self.eog_flag = true;
        self.win_flag = false;

        self.panel.prize = "No prize...".to_string();
        self.panel.value = 0;
        self.panel.bets = 0;
        self.panel.wins = 0;
        self.panel.msg = MSG1.to_string();
    }
}
///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
//	Royal Flush
pub fn royal_flush(nmbr: [usize; 52]) -> bool {
    for i in 0..4 {
        if (nmbr[13 * i] == 1)
            && (nmbr[13 * i + 12] == 1)
            && (nmbr[13 * i + 11] == 1)
            && (nmbr[13 * i + 10] == 1)
            && (nmbr[13 * i + 9] == 1)
        {
            return true;
        }
    }
    return false;
}

// StraightFlush
pub fn straight_flush(nmbr: [usize; 52]) -> bool {
    for j in 0..4 {
        for k in 0..10 {
            if (nmbr[j * 13 + k] == 1)
                && (nmbr[j * 13 + k + 1] == 1)
                && (nmbr[j * 13 + k + 2] == 1)
                && (nmbr[j * 13 + k + 3] == 1)
                && (nmbr[j * 13 + k + 4] == 1)
            {
                return true;
            }
        }
    }
    return false;
}

// FourOfAKind
pub fn four_of_a_kind(nmbr: [usize; 52]) -> bool {
    for i in 0..13 {
        if (nmbr[i] == 1) && (nmbr[i + 13] == 1) && (nmbr[i + 26] == 1) && (nmbr[i + 39] == 1) {
            return true;
        }
    }
    return false;
}

// FullHouse
pub fn full_house(mut nmbr: [usize; 52]) -> bool {
    let mut trio: bool = false;
    let mut pair: bool = false;
    let mut a: usize = 0;
    let mut b: usize = 0;
    let mut c: usize = 0;

    for i in 0..13 {
        if (nmbr[i] == 1) && (nmbr[i + 13] == 1) && (nmbr[i + 26] == 1) {
            trio = true;
            nmbr[i] = 0;
            nmbr[i + 13] = 0;
            nmbr[i + 26] = 0;
            a = i;
            b = i + 13;
            c = i + 26;
        } else if (nmbr[i] == 1) && (nmbr[i + 13] == 1) && (nmbr[i + 39] == 1) {
            trio = true;
            nmbr[i] = 0;
            nmbr[i + 13] = 0;
            nmbr[i + 39] = 0;
            a = i;
            b = i + 13;
            c = i + 39;
        } else if (nmbr[i] == 1) && (nmbr[i + 26] == 1) && (nmbr[i + 39] == 1) {
            trio = true;
            nmbr[i] = 0;
            nmbr[i + 26] = 0;
            nmbr[i + 39] = 0;
            a = i;
            b = i + 26;
            c = i + 39;
        } else if (nmbr[i + 13] == 1) && (nmbr[i + 26] == 1) && (nmbr[i + 39] == 1) {
            trio = true;
            nmbr[i + 13] = 0;
            nmbr[i + 26] = 0;
            nmbr[i + 39] = 0;
            a = i + 13;
            b = i + 26;
            c = i + 39;
        }
    }

    for j in 0..13 {
        if (nmbr[j] == 1) && (nmbr[j + 13] == 1) {
            pair = true;
        } else if (nmbr[j] == 1) && (nmbr[j + 26] == 1) {
            pair = true;
        } else if (nmbr[j] == 1) && (nmbr[j + 39] == 1) {
            pair = true;
        } else if (nmbr[j + 13] == 1) && (nmbr[j + 26] == 1) {
            pair = true;
        } else if (nmbr[j + 13] == 1) && (nmbr[j + 39] == 1) {
            pair = true;
        } else if (nmbr[j + 26] == 1) && (nmbr[j + 39] == 1) {
            pair = true;
        }
    }
    if trio {
        nmbr[a] = 1;
        nmbr[b] = 1;
        nmbr[c] = 1;
    }
    if trio && pair {
        return true;
    } else {
        return false;
    }
}

// Flush
pub fn flush(nmbr: [usize; 52]) -> bool {
    let mut count: usize = 0;
    for j in 0..4 {
        for k in 0..13 {
            if nmbr[13 * j + k] == 1 {
                count += 1;
            }
        }
        if count == 5 {
            return true;
        } else {
            count = 0;
        }
    }
    return false;
}

// Straight
pub fn straight(nmbr: [usize; 52]) -> bool {
    let mut i: usize = 0;
    let mut temp: usize;
    let mut array: [usize; 5] = [0; 5];

    for j in 0..52 {
        if nmbr[j] == 1 {
            array[i] = j;
            i += 1;
        }
    }
    for k in 0..5 {
        array[k] = array[k] % 13;
        if array[k] == 0 {
            array[k] = 13;
        }
    }
    for a in 0..4 {
        for b in a + 1..5 {
            if array[b] < array[a] {
                temp = array[a];
                array[a] = array[b];
                array[b] = temp;
            }
        }
    }
    if (array[4] == array[3] + 1)
        && (array[3] == array[2] + 1)
        && (array[2] == array[1] + 1)
        && (array[1] == array[0] + 1)
    {
        return true;
    }
    if (array[4] == 12)
        && (array[3] == 11)
        && (array[2] == 10)
        && (array[1] == 9)
        && (array[0] == 0)
    {
        return true;
    }
    return false;
}

// ThreeOfAKind
pub fn three_of_a_kind(nmbr: [usize; 52]) -> bool {
    for i in 0..13 {
        if ((nmbr[i] == 1) && (nmbr[i + 13] == 1) && (nmbr[i + 26] == 1))
            || ((nmbr[i] == 1) && (nmbr[i + 13] == 1) && (nmbr[i + 39] == 1))
            || ((nmbr[i] == 1) && (nmbr[i + 26] == 1) && (nmbr[i + 39] == 1))
            || ((nmbr[i + 13] == 1) && (nmbr[i + 26] == 1) && (nmbr[i + 39] == 1))
        {
            return true;
        }
    }
    return false;
}

// TwoPair
pub fn two_pair(nmbr: [usize; 52]) -> bool {
    let mut count: usize = 0;

    for i in 0..13 {
        if (nmbr[i] == 1) && (nmbr[i + 13] == 1) {
            count += 1;
        } else if (nmbr[i] == 1) && (nmbr[i + 26] == 1) {
            count += 1;
        } else if (nmbr[i] == 1) && (nmbr[i + 39] == 1) {
            count += 1;
        } else if (nmbr[i + 13] == 1) && (nmbr[i + 26] == 1) {
            count += 1;
        } else if (nmbr[i + 13] == 1) && (nmbr[i + 39] == 1) {
            count += 1;
        } else if (nmbr[i + 26] == 1) && (nmbr[i + 39] == 1) {
            count += 1;
        }
    }
    if count == 2 {
        return true;
    } else {
        return false;
    }
}

// PairOfAces
pub fn pair_of_aces(nmbr: [usize; 52]) -> bool {
    if (nmbr[0] == 1) && (nmbr[13] == 1) {
        return true;
    } else if (nmbr[0] == 1) && (nmbr[26] == 1) {
        return true;
    } else if (nmbr[0] == 1) && (nmbr[39] == 1) {
        return true;
    } else if (nmbr[13] == 1) && (nmbr[26] == 1) {
        return true;
    } else if (nmbr[13] == 1) && (nmbr[39] == 1) {
        return true;
    } else if (nmbr[26] == 1) && (nmbr[39] == 1) {
        return true;
    } else {
        return false;
    }
}
