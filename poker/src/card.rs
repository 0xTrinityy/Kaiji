#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Copy, Clone)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

#[derive(Debug)]
pub struct pot 
{
    pub pot: f32,
}


#[derive(Debug, Copy, Clone)]
pub struct Card 
{
    pub suit: Suit,
    pub rank: Rank,
}

impl Card 
{
    pub fn new(suit: Suit, rank: Rank) -> Self 
    {
        Card { suit, rank }
    }
}

#[derive(Debug)]
pub struct Player 
{
    pub address: String,
    pub chips_stack : f32,
    pub hands_played : u8,
    pub card_1: Card,
    pub card_2: Card,
}

impl Player
{
    pub fn new(address: String, chips_stack: f32, hands_played: u8, card_1: Card, card_2: Card) -> Self
    {
        Player { address, chips_stack, hands_played, card_1, card_2 }
    }
    pub fn set_card_1(&mut self, card: Card)
    {
        self.card_1 = card;
    }
     pub fn set_card_2(&mut self, card: Card)
    {
        self.card_2 = card;
    }
}