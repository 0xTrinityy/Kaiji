use crate::card::Card;
use crate::card::Player;
use crate::card::pot;
use crate::deck::shuffle_deck;
use crate::card::{Suit, Rank};
use std::io;
use getrandom::getrandom;

pub enum BettingAction 
{
    Fold,
    Check,
    Bet(f32),
    Raise(f32),
}

pub fn parse_betting_action(input: &str) -> BettingAction {
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

pub fn betting_phase(player_1: &mut Player, player_2: &mut Player, current_pot: &mut pot, deck: &mut Vec<Card>) {
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
