use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

// inherent implementation
// fancy term to mean we are adding some functions to our struct
// used to define some methods and associated functions(class methods)
impl Deck {
    fn new() -> Self {
        // list of suits
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];

        // list of values
        let values = [
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
        ];

        let mut cards: Vec<String> = vec![];

        for suit in suits {
            for value in values {
                // the format macro is used to join the two string value and suit
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        // let deck: Deck = Deck { cards };
        // return deck;

        // implicit return (without the semi colon)
        Deck { cards }
    }

    fn shuffle(&mut self) {
        // create a new random number generator
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        // implicit return
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    //deck.shuffle();
    let cards = deck.deal(10);
    println!("Here is your hand: {:#?}", cards);
    println!("Here is your deck: {:#?}", deck);
}

/* *

 Note: The Deck type doesn't automatically have a way to describe itself in the {:?} debugging
 format unless it implements the Debug trait.

 Solution:
    Rust suggests you derive the Debug trait for your Deck struct or implement the Debug trait manually.
     This will enable println! to use the {:?} specifier with deck.

    1- The Debug trait allows you to use {:?} and {:#?} for debugging-friendly output.
    2- Use #[derive(Debug)] for quick and automatic implementation.
    3- Manually implementing Debug gives you full control over how a type is displayed.


 In Rust bindings are immutable (can't change) be default:

 let numbers = vect![];

 numbers.push(1) // this is wrong we cannot change the value
 numbers = vect1[] // this is wrong too , we cannot reassign


 To make something immutable:
 we need to prefix it with mut

 let mut numbers =  vect![];
 numbers.push(1) // now we are allowed to change values

 numbers = vect![] // we can even reassign

{:#?} ==> format things better on the screen


// Inherent implementation

impl Deck {

// Associated function: whenever we have some functionality
// not tied to  a specific instance
// we dont add &self
  fn new() --> Self {

  //...

  }

  // Method that operate on specific instance
  // we have to add &self as first parameter to this function
  fn suffle(&self) {

  //...

  }



}

*/
