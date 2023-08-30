mod card;

use card::{Card, Suit, Rank, Player, pot, public_cards};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Hand 
{
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

pub fn has_royal_flush(cards: &[Card]) -> bool {
    let mut suited_cards: Vec<&Card> = Vec::new();

    for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
        let suited = cards.iter().filter(|&&card| card.suit == *suit).collect::<Vec<_>>();
        suited_cards.extend(suited);
    }

    for suited in suited_cards {
        let mut found_10 = false;
        let mut found_jack = false;
        let mut found_queen = false;
        let mut found_king = false;
        let mut found_ace = false;

        for card in cards.iter() {
            if card.suit == suited.suit {
                match card.rank {
                    Rank::Number(10) => found_10 = true,
                    Rank::Jack => found_jack = true,
                    Rank::Queen => found_queen = true,
                    Rank::King => found_king = true,
                    Rank::Ace => found_ace = true,
                    _ => {}
                }
            }
        }

        if found_10 && found_jack && found_queen && found_king && found_ace {
            return true;
        }
    }

    false
}

pub fn has_straight_flush(cards: &[Card]) -> bool {
    let mut suit_count: Vec<Vec<Rank>> = vec![];

    for _ in 0..4 {
        suit_count.push(vec![]);
    }

    for card in cards {
        match card.suit {
            Suit::Hearts => suit_count[0].push(card.rank),
            Suit::Diamonds => suit_count[1].push(card.rank),
            Suit::Clubs => suit_count[2].push(card.rank),
            Suit::Spades => suit_count[3].push(card.rank),
        }
    }

    for ranks in suit_count.iter() {
        if ranks.len() < 5 {
            continue;
        }

        let mut sorted_ranks = ranks.to_vec();
        sorted_ranks.sort_by_key(|rank| match rank {
            Rank::Number(value) => *value,
            _ => 0, 
        });                 /// this is for non-numerical ranks

        let mut straight_count = 0;
        let mut prev_rank = Rank::Number(0);

        for rank in sorted_ranks.iter().rev() {
            if *rank == prev_rank {
                continue;
            }

            if let Rank::Number(rank_value) = *rank {
                if prev_rank == Rank::Number(rank_value + 1) {
                    straight_count += 1;
                    if straight_count == 4 {
                        return true;
                    }
                } else {
                    straight_count = 0;
                }
                prev_rank = *rank;
            }
        }
    }
    false
}

pub fn has_four_of_a_kind(cards: &[Card]) -> bool {
    let mut rank_count: Vec<u8> = vec![0; 13];

    for card in cards {
        match card.rank {
            Rank::Number(value) => rank_count[(value - 1) as usize] += 1,
            _ => (),
        }
    }
    rank_count.iter().any(|&count| count >= 4)
}

pub fn has_full_house(cards: &[Card]) -> bool 
{
    if (has_three_of_a_kind(cards) && has_one_pair(cards)) 
    {
        return true;
    }
    false
}

pub fn has_flush(cards: &[Card]) -> bool
{
        


    false 
}

pub fn evaluate_hand(cards: &[Card]) -> Hand 
{
    // Here, i have to implement the logic that evaluate hand from the most powerfull to the weakest and compare the value found for each player
    if has_royal_flush(cards) 
    {
        return (2000);
    } 
    else if has_straight_flush(cards)
    {
        return (1800);
    }
    else if has_four_of_a_kind(cards)
    {
        return (1600);
    }
    else if has_full_house(cards)
    {
        return (1400);
    }
    else if has_flush(cards)
    {
        return (1200);
    }
    else if has_straight(cards)
    {
        return (1000);
    }
    else if has_three_of_a_kind(cards)
    {
        return (800);
    }
    else if has_two_pair(cards)
    {
        return (600);
    }
    else if has_one_pair(cards)
    {
        return (400);
    }
    else
    {
        return (has_high_card(cards)); 
    }
}

pub fn compare_hands(player1_cards: &[Card], player2_cards: &[Card], public_cards: &[Card]) 
{
    let player1_best_hand = evaluate_hand(&[player1_cards[0], player1_cards[1], public_cards[0], public_cards[1], public_cards[2], public_cards[3], public_cards[4]]);
    let player2_best_hand = evaluate_hand(&[player2_cards[0], player2_cards[1], public_cards[0], public_cards[1], public_cards[2], public_cards[3], public_cards[4]]);
    
    if player1_best_hand > player2_best_hand 
    {
        println!("Player 1 wins with {:?}", player1_best_hand);
    } 
    else if player2_best_hand > player1_best_hand 
    {
        println!("Player 2 wins with {:?}", player2_best_hand);
    } 
    else 
    {
        println!("It's a tie!");
    }
}