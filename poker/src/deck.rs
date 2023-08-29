use crate::card::{Card, Suit, Rank};
use rand::Rng;

pub fn shuffle_deck(deck: &mut Vec<Card>) 
{
    let mut rng = rand::thread_rng();
    for i in (1..deck.len()).rev() 
    {
        let j = rng.gen_range(0..=i);
        deck.swap(i, j);
    }
}
