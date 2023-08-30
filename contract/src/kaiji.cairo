#[starknet::interface]
trait IKaiji<TContractState> {
    fn setPlayer(ref self: TContractState, user: felt252) -> bool;
    fn  setCard(ref self: TContractState, user: felt252, card: u32, proof: u32) -> bool;
    fn  getCard(ref self: TContractState, user: felt252, cardNb: u32) -> u32;
}

#[starknet::contract]
mod Kaiji {
    use core::traits::AddEq;
    use option::OptionTrait;
    use box::BoxTrait;
    use array::ArrayTrait;
    use array::SpanTrait;

    #[derive(Copy, Drop)]
    struct User {
        pk: felt252,
        index: u32,
        card1: u32,
        card2: u32,
    }

    #[storage]
    struct Storage {
        players: Array<User>,
        cards: Array<u32>,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        let mut players = ArrayTrait::<User>::new();
        let mut cards = ArrayTrait::<u32>::new();
    }

    #[external(v0)]
    impl Kaiji of super::IKaiji<ContractState>{
        fn setPlayer(ref self: ContractState, user: felt252) -> bool {
        let mut i = 0;
        let players_span: Span<User> = self.players.read().span();

        match self.players.read().get(5) {
            Option::Some(_) => {
                return false;
            },
            Option::None(_) => {}
        }
        let j: bool = loop{
            match players_span.get(i) {
                Option::Some(_) => {
                    if i > 5{
                        break false;
                    }
                },
                Option::None(_) => {
                    if (i <= 5) {
                        break true;
                    }
                }
            }
            i += 1;
        };
        if !j {
            return false;
        }
        let mut player: User = User{
            pk: user,
            index: i,
            card1: 0,
            card2: 0,
        };
        let mut tmp: Array<User> = self.players.read();
        tmp.append(player);
        self.players.write(tmp);
        true
    }

    fn  setCard(ref self: ContractState, user: felt252, mut card: u32, proof: u32) -> bool {
        let mut i = 0;
        let players_span: Span<User> = self.players.read().span();
        let cards_span: Span<u32> = self.cards.read().span();
        let mut player: User = User{
            pk: 0,
            index: 0,
            card1: 0,
            card2: 0,
        };
        loop {
            if user == *players_span.get(i).unwrap().unbox().pk {
                player = *players_span.get(i).unwrap().unbox();
                break;
            }
            i += 1;
        };
        if proof != 0 {
            return false;
        };
        match self.cards.read().get(0) {
            Option::Some(_) => {
                return loop {
                    match cards_span.get(i) {
                        Option::Some(_) => {
                            if card == *cards_span.get(i).unwrap().unbox() {
                                break false;
                            }
                        },
                        Option::None(_) => {
                            break true;
                        }
                    }
                    i += 1;
                };
            },
            Option::None(_) => {
                if player.card1 == 0 {
                    player.card1 = card;
                } else {
                    player.card2 = card;
                }
            }
        }
        return true;
    }

    fn  getCard(ref self: ContractState, user: felt252, cardNb: u32) -> u32 {
        let players_span: Span<User> = self.players.read().span();
        let mut i = 0;
        let mut player: User = User{
            pk: 0,
            index: 0,
            card1: 0,
            card2: 0,
        };
        loop {
            if user == *players_span.get(i).unwrap().unbox().pk {
                player = *players_span.get(i).unwrap().unbox();
                break;
            }
            i += 1;
        };
        if cardNb == 1 {
            return player.card1;
        } else {
            return player.card2;
        }
    }
    }
}
//fn  main(){
//    let mut players = ArrayTrait::new();
//    let mut cards = ArrayTrait::new();
//    let mut user: felt252 = 'test';
//    let i = 0;
//
//    setPlayer(user, ref players);
//    setCard(user, 7, ref players, 1, ref cards);
//    getCard(user, ref players, 1).print();
//    assert(*players.get(0).unwrap().unbox().index == 0, 'it s not ok');
//    return ();
//}