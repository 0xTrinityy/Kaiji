
/*
    1er etape : Definir un deck de 52 cartes 
    2eme etape : Definir 2 joueurs
    3eme etape : le programme a les 52 cartes
    4eme etape : On chosit une carte au hasard
    5eme etape : On donne la carte au joueur 1 sous forme encrypter en lui retournant la structure qui permet de decrypter la carte
    6eme etape : On donne la carte au joueur 2 sous forme encrypter en lui retournant la structure qui permet de decrypter la carte
    x2
*/

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

#[derive(Debug)]
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
    let mut player_1 = Player::new("0x000".to_string(), 100, 0, Card::new(Suit::Hearts, Rank::Number(2)), Card::new(Suit::Hearts, Rank::Number(3)));

    println!("{:?}\n", player_1);

}