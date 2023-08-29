mod card;
mod deck;
mod betting_phase;

use card::{Card, Suit, Rank, Player, pot};
use deck::shuffle_deck;
use betting_phase::{BettingAction, betting_phase};
use std::io;
use getrandom::getrandom;


fn main() 
{
    ///////////////////////////////// * Deck created and shuffled * /////////////////////////////////

    let mut deck = Vec::new();
    let mut table_pot = pot { pot: 0.0 };
    for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
        for rank in &[
            Rank::Ace,
            Rank::Number(2),
            Rank::Number(3),
            Rank::Number(4),
            Rank::Number(5),
            Rank::Number(6),
            Rank::Number(7),
            Rank::Number(8),
            Rank::Number(9),
            Rank::Number(10),
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ] 
        {
            let card = Card::new(*suit, *rank);
            deck.push(card);
        }
    }
    println!("\n deck\n");
    println!("{:?}", deck);
    println!("\n\n");

    shuffle_deck(&mut deck);

    println!("\n Shuffled deck\n");
    println!("{:?}", deck);
    println!("\n\n");

    ///////////////////////////////// * Players got random cards * /////////////////////////////////

    //// * Player 1 * ////

    let mut player_1 = Player::new("0x000".to_string(), 100.0, 0, Card::new(Suit::Hearts, Rank::Number(0)), Card::new(Suit::Hearts, Rank::Number(0)));

    let mut buffer = [0u8; 4];
    getrandom(&mut buffer).expect("Failed to generate");
    let random_number = u32::from_le_bytes(buffer);
    println!("Random number: {}", (random_number % deck.len() as u32));

    let final_card_index = (random_number % deck.len() as u32) as usize;
    let final_card = deck.remove(final_card_index);
    println!("final card: {:?}", final_card);
    player_1.set_card_1(final_card);

    let mut buffer_2 = [0u8; 4];
    getrandom(&mut buffer_2).expect("Failed to generate");
    let random_number_2 = u32::from_le_bytes(buffer_2);
    println!("Random number: {}", (random_number_2 % deck.len() as u32));

    let final_card_2_index = (random_number_2 % deck.len() as u32) as usize;
    let final_card_2 = deck.remove(final_card_2_index);
    println!("final card: {:?}\n", final_card_2);
    player_1.set_card_2(final_card_2);

    println!("{:?}\n", player_1);

    //// * Player 2 * ////

    let mut player_2 = Player::new("0x001".to_string(), 100.0, 0, Card::new(Suit::Hearts, Rank::Number(0)), Card::new(Suit::Hearts, Rank::Number(0)));


    let mut buffer_3 = [0u8; 4];
    getrandom(&mut buffer_3).expect("Failed to generate");
    let random_number_3 = u32::from_le_bytes(buffer_3);
    println!("Random number: {}", (random_number_3 % deck.len() as u32));

    let final_card_3_index = (random_number_3 % deck.len() as u32) as usize;
    let final_card_3 = deck.remove(final_card_3_index);
    println!("final card: {:?}", final_card_3);
    player_2.set_card_1(final_card_3);

    let mut buffer_4 = [0u8; 4];
    getrandom(&mut buffer_4).expect("Failed to generate");
    let random_number_4 = u32::from_le_bytes(buffer_4);
    println!("Random number: {}", (random_number_4 % deck.len() as u32));

    let final_card_4_index = (random_number_4 % deck.len() as u32) as usize;
    let final_card_4 = deck.remove(final_card_4_index);
    println!("final card: {:?}\n", final_card_4);
    player_2.set_card_2(final_card_4);

    println!("{:?}\n", player_2);

    ///////////////////////////////// * Public cards and betting phase  * /////////////////////////////////

    betting_phase(&mut player_1, &mut player_2, &mut table_pot, &mut deck);

    return ();
}