//extern crate getrandom;
use std::io;
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

enum BettingAction 
{
    Fold,
    Check,
    Bet(f32),
    Raise(f32),
}

#[derive(Debug)]
struct Public_Card {
    Public_Card_1: Card,
    Public_Card_2: Card,
    Public_Card_3: Card,
    Public_Card_4: Card,
    Public_Card_5: Card,
}

#[derive(Debug)]
struct pot 
{
    pot: f32,
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

fn parse_betting_action(input: &str) -> BettingAction {
    match input.trim().to_lowercase().as_str() {
        "f" => BettingAction::Fold,
        "c" => BettingAction::Check,
        "b" => {
            println!("Enter the amount to bet:");
            let mut amount = String::new();
            io::stdin().read_line(&mut amount).expect("Failed to read line");
            let amount = amount.trim().parse::<f32>().unwrap_or(0.0);
            BettingAction::Bet(amount)
        }
        "r" => {
            println!("Enter the amount to raise to:");
            let mut amount = String::new();
            io::stdin().read_line(&mut amount).expect("Failed to read line");
            let amount = amount.trim().parse::<f32>().unwrap_or(0.0);
            BettingAction::Raise(amount)
        }
        _ => BettingAction::Check,
    }
}

fn betting_phase(player_1: &mut Player, player_2: &mut Player, current_pot: &mut pot, deck: &mut Vec<Card>) {
    let mut current_bet = 0.0;
    let mut player_1_action = BettingAction::Check;
    let mut player_2_action = BettingAction::Check;

    loop 
    {
        println!("Player 1's turn: Choose action (f, c, b, r):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        player_1_action = parse_betting_action(&input);
    
        println!("Player 2's turn: Choose action (f, c, b, r):");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        player_2_action = parse_betting_action(&input);
    
            // Process actions, update pot and player chip stacks accordingly
    
        if let BettingAction::Fold = player_1_action
        {
            println!("Player 1 folds.");
            return;
        }
    
        if let BettingAction::Fold = player_2_action 
        {
            println!("Player 2 folds.");
            return;
        }
    
        loop 
        {
            println!("Player 1's turn: Choose action (f, c, b, r):");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            player_1_action = parse_betting_action(&input);
            
            println!("Player 2's turn: Choose action (f, c, b, r):");
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            player_2_action = parse_betting_action(&input);
            
                // Process actions, update pot and player chip stacks accordingly
            
            if let BettingAction::Fold = player_1_action 
            {
                println!("Player 1 folds.");
                return;
            }
            
            if let BettingAction::Fold = player_2_action 
            {
                println!("Player 2 folds.");
                return;
            }
            
            match (player_1_action, player_2_action) 
            {
                (BettingAction::Check, BettingAction::Check) => 
                {
                    println!("Both players check.");
                    // No bet, update pot if needed
                }
                (BettingAction::Bet(amount_1), BettingAction::Check) => 
                {
                    println!("Player 1 bets: {}", amount_1);
                    current_bet = amount_1;
                    player_1.chips_stack -= amount_1;
                    current_pot.pot += amount_1;
                }
                (BettingAction::Check, BettingAction::Bet(amount_2)) => 
                {
                    println!("Player 2 bets: {}", amount_2);
                    current_bet = amount_2;
                    player_2.chips_stack -= amount_2;
                    current_pot.pot += amount_2;
                }
                (BettingAction::Bet(amount_1), BettingAction::Bet(amount_2)) => 
                {
                    let max_bet = amount_1.max(amount_2);
                    println!("Both players bet: {}", max_bet);
                    current_bet = max_bet;
                    player_1.chips_stack -= amount_1;
                    player_2.chips_stack -= amount_2;
                    current_pot.pot += amount_1 + amount_2;
                }
                (BettingAction::Raise(amount_1), BettingAction::Raise(amount_2)) => 
                {
                    let max_raise = amount_1.max(amount_2);
                    println!("Both players raise to: {}", max_raise);
                    current_bet = max_raise;
                    player_1.chips_stack -= amount_1;
                    player_2.chips_stack -= amount_2;
                    current_pot.pot += amount_1 + amount_2;
                }
                    // Handle other cases
                _ => 
                {
                    println!("Invalid combination of actions. Players need to agree on an action.");
                    continue; // Restart the loop
                }
            }
        }
    }
}

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

}