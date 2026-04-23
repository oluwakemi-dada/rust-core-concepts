use rand::seq::SliceRandom;
use rand::{rng, Rng};

#[derive(Copy, Clone, Debug)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

#[derive(Copy, Clone, Debug)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}

#[derive(Debug)]
struct Card {
    suit: Option<Suit>,
    rank: Rank,
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let suits = [Suit::Clubs, Suit::Spades, Suit::Hearts, Suit::Diamonds];
        let ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];
        let mut cards = Vec::with_capacity(52);

        for suit in suits.into_iter() {
            for rank in ranks.into_iter() {
                cards.push(Card {
                    suit: Some(suit),
                    rank,
                });
            }
        }

        Self { cards }
    }

    fn shuffle(&mut self) {
        let mut my_rng = rng();
        self.cards.shuffle(&mut my_rng);
    }

    fn insert_jokers(&mut self) {
        let mut my_rng = rng();
        for _ in 0..2 {
            let random_index = my_rng.random_range(0..self.cards.len());
            let joker_card = Card {
                suit: None,
                rank: Rank::Joker,
            };
            self.cards.insert(random_index, joker_card);
        }
    }

    fn delete_random_card(&mut self) {
        let mut my_rng = rng();
        let should_delete_card = my_rng.random_bool(0.65);
        if should_delete_card {
            let random_index = my_rng.random_range(0..self.cards.len());
            self.cards.remove(random_index);
        }
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    deck.insert_jokers();

    for _ in 0..10 {
        deck.delete_random_card();
    }

    println!("{:#?}", deck.cards.len());
}

// ---------------------------------------------------- //

// use rand::random;

// fn main() {
//     let random_float: f64 = random();
//     println!("{}", random_float * 100.0);

//     let random_int = random::<u8>();
//     println!("{}", random_int);
// }

// ---------------------------------------------------- //

// use rand::{rng, Rng};

// fn main() {
//   let mut my_rng = rng();
//   let random_float = my_rng.random::<f64>();
//   println!("{}", random_float);

//   let ten_random_values = (0..10).map(|_| my_rng.random::<i8>()).collect::<Vec<i8>>();
//   println!("{:?}", ten_random_values);

//   let random_number: i32 = my_rng.random_range(29..53);
//   println!("{random_number}",);

//   println!("{}", my_rng.random_bool(0.9));

//   println!("{}", my_rng.random_ratio(1, 2));
//   println!("{}", my_rng.random_ratio(9, 13));
// }

// ---------------------------------------------------- //

// use rand::rng;
// use rand::seq::SliceRandom;

// fn main() {
//     let mut my_rng = rng();
//     let mut candies = vec![
//         "Sour Patch Kids",
//         "Kit Kat",
//         "Twix",
//         "Snickers",
//         "Starburst",
//     ];
//     candies.shuffle(&mut my_rng);

//     println!("{:?}", candies);
// }
