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
   pub gameid: u8, // oyun numarasi  
   pub number_of_participants: u8, // katilimci sayisi 
   pub winning_numbers: [u8;5], // kazanan numaralar 
   pub number_of_winner: u8, // kazanan kisi sayisi
   pub prize_pool: u64, // odul havuzu 
   pub prize_amount: u64, // her bir kazananin alacagi odul miktari 
   pub is_active: u8, // oyun aktif mi 
   pub ticket_money: u64,
} 

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct GameCount{
    pub game_count: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct DrawData {
    pub prize_amount: u64,
    pub winning_numbers: [u8; 5],
}




