use borsh::{BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Ticket{
    pub gameid: u8,
    pub user_address: [u8;32],
    pub participant_numbers: [u8;5], // katilimci numaralari (katilimcinin tahmin ettigi sayilar gibi
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct LottoGame{
   pub gameid: u8, // oyun numarasi 1
   pub number_of_participants: u8, // katilimci sayisi 1
   pub winning_numbers: [u8;5], // kazanan numaralar 5
   pub winner_limit: u8, // kac kisi kazanbilir 1
   pub prize_pool: u64, // odul havuzu 8
   pub prize_amount: u64, // her bir kazananin alacagi odul miktari 8
   pub is_active: u8 // oyun aktif mi  1
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct GameCount{
    pub game_count: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Authority{
    pub authority_accounts:[u8;32],
}

