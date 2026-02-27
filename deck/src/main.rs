use rand::seq::SliceRandom;

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        // List of 'suits' - 'hearts', 'diamonds', 'clubs', 'spades'
        let suits = vec!["hearts", "diamonds", "clubs", "spades"];

        // list of 'values' - 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine', 'ten', 'jack', 'queen', 'king', 'ace'
        let values = vec![
            "ace", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack",
            "queen", "king",
        ];

        let mut cards = Vec::new();

        // Double nested for loop
        for suit in &suits {
            for value in &values {
                // println!("{} of {}", value, suit);
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // let deck: Deck = Deck { cards: vec![] };
        // let deck: Deck = Deck { cards: Vec::new() };
        // let deck: Deck = Deck { cards: crds };
        // let deck: Deck = Deck { cards };

        // return deck;

        // return Deck { cards };

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    // println!("Here is your deck: {:?}", deck);
    println!("Here is your deck: {:#?}", deck);
    // println!("Here is your deck: {deck:?}")

    deck.shuffle();

    println!("Here is your shuffled deck: {deck:#?}");

    let hand = deck.deal(5);
    println!("Your hand: {hand:#?}");
}
