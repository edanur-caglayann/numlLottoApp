use crate::{error::RNGProgramError::InvalidInstruction, state::DrawData, };
use borsh::BorshDeserialize;
use solana_program::{instruction::InstructionError, msg, program_error::ProgramError};

#[derive(Debug, PartialEq)]
pub enum RNGProgramInstruction { 
  GameCount,
  CreateLottoGame,
  Ticket{participant_numbers: [u8; 5]},
  Draw{prize_amount:u64, winning_numbers: [u8; 5]},
  ClaimPrize,
}

impl RNGProgramInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
  
      let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
       
      Ok(match tag {
        1 => Self::GameCount,
        2 => Self::CreateLottoGame,
        3 => {
          if rest.len() < 5 {
            return Err(InvalidInstruction.into());
          }
          let mut arr = [0u8; 5];
          arr.copy_from_slice(&rest[..5]);
          Self::Ticket { participant_numbers: arr }
      }
      4 => {
          let draw_data = DrawData::try_from_slice(&rest).map_err(|_|InvalidInstruction)?;
          Self::Draw {
              prize_amount: draw_data.prize_amount,
              winning_numbers: draw_data.winning_numbers,
          }
      }
        5 => Self::ClaimPrize,
        _ => return Err(InvalidInstruction.into()),
      })
    }
  }
  
  