
/*
    1er etape : Definir un deck de 52 cartes 
    2eme etape : Definir 2 joueurs
    3eme etape : le programme a les 52 cartes
    4eme etape : On chosit une carte au hasard
    5eme etape : On donne la carte au joueur 1 sous forme encrypter en lui retournant la structure qui permet de decrypter la carte
    6eme etape : On donne la carte au joueur 2 sous forme encrypter en lui retournant la structure qui permet de decrypter la carte
    x2
*/
//extern crate getrandom;
use rand::Rng;
use rand::seq::SliceRandom;
use getrandom::getrandom;

#[derive(Debug, Copy, Clone)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Copy, Clone)]
enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone)]
struct Card {
    suit: Suit,
    rank: Rank,
}

#[derive(Debug)]
struct Player {
    address: String,
    chips_stack : f32,
    hands_played : u8,
    card_1: Card,
    card_2: Card,
}

impl Card 
{
    fn new(suit: Suit, rank: Rank) -> Self 
    {
        Card { suit, rank }
    }
}

impl Player
{
    fn new(address: String, chips_stack: f32, hands_played: u8, card_1: Card, card_2: Card) -> Self
    {
        Player { address, chips_stack, hands_played, card_1, card_2 }
    }
    fn set_card_1(&mut self, card: Card)
    {
        self.card_1 = card;
    }
    fn set_card_2(&mut self, card: Card)
    {
        self.card_2 = card;
    }
}

fn shuffle_deck(deck: &mut Vec<Card>)
{
    let mut rng = rand::thread_rng();
    for i in (1..deck.len()).rev() 
    {
        let j = rng.gen_range(0..=i);
        deck.swap(i, j);
    }
}

fn main() {
    let mut deck = Vec::new();

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

    let mut player_1 = Player::new("0x000".to_string(), 100.0, 0, Card::new(Suit::Hearts, Rank::Number(2)), Card::new(Suit::Hearts, Rank::Number(3)));

    println!("{:?}\n", player_1);

    let mut buffer = [0u8; 4];
    getrandom(&mut buffer).expect("Failed to generate");
    let random_number = u32::from_le_bytes(buffer);
    println!("Random number: {}", (random_number % 52));

    let final_card = &deck[(random_number % 52) as usize];
    println!("final card: {:?}", final_card);
    player_1.set_card_1(*final_card);


    let mut buffer_2 = [0u8; 4];
    getrandom(&mut buffer).expect("Failed to generate");
    let random_number_2 = u32::from_le_bytes(buffer);
    println!("Random number: {}", (random_number_2 % 51));
    
    let final_card_2= &deck[(random_number % 51) as usize];
    println!("final card: {:?}", final_card_2);
    player_1.set_card_2(*final_card_2);

}