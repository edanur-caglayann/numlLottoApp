use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum RNGProgramError {
  #[error("Invalid Instruction")]
  InvalidInstruction,

  #[error("Arithmetic Err")]
  ArithmeticErr,

  #[error("The game is not active")]
  NotActiveErr,

  #[error("Authority Error")]
  AuthorityError,

  #[error("Ownership Error")]
  OwnershipError,

  #[error("GameId Mismatch Error")]
  GameIdMismatchError,

  #[error("No Prize Error")]
  NoPrizeError,

  #[error("Participant Limit Reached Error")]
  ParticipantLimitReachedError,

  #[error("Already active Error")]
  AlreadyActiveError,

  #[error("Ticket fee not paid")]
  TicketFeeNotPaidError,

  #[error("You didn't win")]
  YouNotWinnerError,

  #[error("Update Error")]
  UpdateError,

  #[error("Insufficient Funds Error")]
  InsufficientFundsError,
  
}

impl From<RNGProgramError> for ProgramError {
  fn from(e: RNGProgramError) -> Self {
    ProgramError::Custom(e as u32)
  }
}