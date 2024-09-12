use crate::{error::RNGProgramError::InvalidInstruction, state::{}, };
use borsh::BorshDeserialize;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug, PartialEq)]
pub enum RNGProgramInstruction { 
  GameCount,
  CreateLottoGame,
  StartGame,
  Ticket,
  Draw{prize_amount:u64, winning_numbers: [u8; 5]},
  ClaimPrize,
}

impl RNGProgramInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
  
      let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
       
      Ok(match tag {
        1 => Self::GameCount,
        2 => Self::CreateLottoGame,
        3 => Self::StartGame,
        4 => Self::Ticket,
        5 => {
          let (prize_amount_bytes, winning_numbers_bytes) = rest.split_at(8); // 8 byte'lik parca ve kalan kismi olarak ikiye ayirdik
          let prize_amount = u64::from_le_bytes(prize_amount_bytes.try_into().map_err(|_| InvalidInstruction)?); // kalan kismi wn icin kullandik. 5 byte'lik dizi
          let winning_numbers: [u8; 5] = winning_numbers_bytes[..5].try_into().map_err(|_| InvalidInstruction)?;
          Self::Draw { prize_amount, winning_numbers }
      },
        6 => Self::ClaimPrize,
        _ => return Err(InvalidInstruction.into()),
      })
    }
  }
  
  