use core::borrow;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{ 
    account_info::{next_account_info, AccountInfo}, clock, config, entrypoint::ProgramResult, lamports, msg, program::{invoke, invoke_signed}, program_error::ProgramError, pubkey::{self, Pubkey, PubkeyError}, rent::Rent, system_instruction::{self}, system_program, sysvar::Sysvar
    };
    use crate::{instruction::RNGProgramInstruction, state::{Authority, GameCount, LottoGame, Ticket}};
    use crate::error::RNGProgramError::{InvalidInstruction, ArithmeticErr, NotActiveErr, AuthorityError, GameIdMismatchError, NoPrizeError, OwnershipError, ParticipantLimitReachedError};
    pub struct Processor;
    impl Processor {
    pub fn process(
      _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
      ) -> ProgramResult {
        let instruction: RNGProgramInstruction = RNGProgramInstruction::unpack(instruction_data)?;
    
    
        match instruction { 
          RNGProgramInstruction:: CreateLottoGame => {
            Self::create_lotto_game( accounts,_program_id)
             },
          RNGProgramInstruction:: GameCount => {
              Self::game_count( accounts,_program_id)
             },
          RNGProgramInstruction:: StartGame => {
              Self::start_game( accounts,_program_id)
             }, 
          RNGProgramInstruction:: Participate => {
              Self::participate( accounts,_program_id)
             },  
          RNGProgramInstruction:: Draw{prize_amount,winning_numbers }  => {
            Self::draw( accounts,_program_id,prize_amount,winning_numbers)
             },
          RNGProgramInstruction:: ClaimPrize => {
              Self::claim_prize( accounts,_program_id)
             },  
          RNGProgramInstruction:: CreateAuthority => {
              Self::create_authority( accounts,_program_id)
             },
        }
      } 
      
      
      // yeni oyun olusturma
      pub fn create_lotto_game (
        accounts: &[AccountInfo],
        program_id: &Pubkey,
      ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let count = next_account_info(account_info_iter)?;
        let lotto_game = next_account_info(account_info_iter)?;

        if !payer.is_signer{ 
          msg!("payer is not a signer");
          return Err(AuthorityError.into());
        }
  
        let mut count_data = GameCount::try_from_slice(&count.data.borrow())?;
        count_data.game_count = count_data.game_count.checked_add(1).ok_or(ArithmeticErr)?;

        let(lotto_game_pda, bump) = Pubkey::find_program_address(&[b"lotto_game",count_data.game_count.to_string().as_ref()], program_id);

        let rent = Rent:: default();
        let game_rent = rent.minimum_balance(25);

        invoke_signed(
          &system_instruction::create_account(payer.key, &lotto_game_pda, game_rent, 25, program_id), 
          &[lotto_game.clone(),payer.clone()], 
          &[
            &[b"lotto_game",count_data.game_count.to_string().as_ref(),&[bump]]
          ]
        )?;

        let lotto_game_info = LottoGame {
            gameid: count_data.game_count,
            number_of_participants: 0,
            winning_numbers:[0; 5],
            winner_limit: 10,
            prize_pool: 0,
            prize_amount: 0,
            is_active: 1,
        };

        lotto_game_info.serialize(&mut &mut lotto_game.try_borrow_mut_data()?[..])?;
        count_data.serialize(&mut &mut count.try_borrow_mut_data()?[..])?;
        Ok(())
      }

      // oyun sayisini takip 
      pub fn game_count (
        accounts: &[AccountInfo],
        program_id: &Pubkey,
      ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let count = next_account_info(account_info_iter)?;

        let rent = Rent::default();
        let count_rent = rent.minimum_balance(1);

        let(count_address, bump) = Pubkey::find_program_address(&[b"count"], program_id);

        invoke_signed(
          &system_instruction::create_account(payer.key, &count_address,count_rent , 1, program_id),
          &[count.clone(),payer.clone()],
          &[
            &[b"count", &[bump]]
          ]
          )?;
        Ok(())
      }
      
      // oyun baslatma
      pub fn start_game (
        accounts: &[AccountInfo],
        program_id: &Pubkey,
      ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let lotto_game = next_account_info(account_info_iter)?;

        let mut lotto_game_data = LottoGame::try_from_slice(&lotto_game.data.borrow())?;
        
        if !payer.is_signer{ 
          msg!("payer is not a signer");
          return Err(AuthorityError.into());
        }
        
        // oyunu baslatalim
        lotto_game_data.is_active = 1; 

        // oyun numarasini arttiralim
        lotto_game_data.gameid = lotto_game_data.gameid.checked_add(1).ok_or(ArithmeticErr)?;


        lotto_game_data.serialize(&mut &mut lotto_game.try_borrow_mut_data()?[..])?;

        Ok(())
      }
    
      // katilim fonks-bilet olsuturma
      pub fn participate (
        accounts: &[AccountInfo],
        program_id: &Pubkey,
      ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?; 
        let lotto_game = next_account_info(account_info_iter)?;
        let ticket =  next_account_info(account_info_iter)?;

        let mut lotto_game_data = LottoGame::try_from_slice(&lotto_game.data.borrow())?;

        if lotto_game_data.is_active == 0 {
          msg!("The game is not active");
          return Err(ArithmeticErr.into());
        }

        //kullanici sayisi kontroli???
        
        let ticket_info = Ticket {
            gameid: lotto_game_data.gameid,
            user_address:payer.key.to_bytes(),
            participant_numbers:[0; 5],
        };

        // Katilimci sayisini arttiralim
        lotto_game_data.number_of_participants = lotto_game_data.number_of_participants.checked_add(1).ok_or(ArithmeticErr)?; 
       
        ticket_info.serialize(&mut &mut ticket.try_borrow_mut_data()?[..])?;
        lotto_game_data.serialize(&mut &mut lotto_game.try_borrow_mut_data()?[..])?;
        Ok(())
      }
    
      // cekilis
      pub fn draw (
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        prize_amount: u64,
        winning_numbers: [u8; 5], //kazanan numaralari parametre olarak al
      ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let authority  = next_account_info(account_info_iter)?;
        let lotto_game   = next_account_info(account_info_iter)?;

           // otorite imzalayici mi
        if !authority.is_signer {
          return Err(ProgramError::MissingRequiredSignature);
           }

        if lotto_game.owner!= program_id {
            msg!("");
          }

        let mut lotto_game_data = LottoGame::try_from_slice(&lotto_game.data.borrow())?; 

       //odul aktif degilse odul verilemez
        if lotto_game_data.is_active == 0 {
          msg!("");
        }
        
        // odulu guncelleme
        lotto_game_data.prize_amount = prize_amount;
        lotto_game_data.winning_numbers = winning_numbers; 
        
        lotto_game_data.serialize(&mut &mut lotto_game.try_borrow_mut_data()?[..])?;
        Ok(())
      }

      // odul dagitma fonks
      pub fn claim_prize (
        accounts: &[AccountInfo],
        program_id: &Pubkey,
      ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let authority  = next_account_info(account_info_iter)?;
        let lotto_game   = next_account_info(account_info_iter)?;
        let ticket =  next_account_info(account_info_iter)?;

           // otorite imzalayici mi
           if !authority.is_signer {
            msg!("Authority is not a signer");
            return Err(AuthorityError.into());
             }

           if lotto_game.owner!= program_id {
            msg!("not a program for authority");
            return Err(OwnershipError.into());
            }

           if ticket.owner!= program_id {
            msg!("not a program for authority");
            return Err(OwnershipError.into());
            }

          let mut lotto_game_data = LottoGame::try_from_slice(&lotto_game.data.borrow())?; 
          let mut ticket_data = Ticket::try_from_slice(&ticket.data.borrow())?; 

            if ticket_data.gameid != lotto_game_data.gameid{
              msg!("Ticket does not match this lotto game.");
              return Err(GameIdMismatchError.into());
            }
            
            if lotto_game_data.prize_amount == 0 {
              msg!("The prize amount of the lotto game is zero.");
              return Err(NoPrizeError.into());
            }
          
          if lotto_game_data.number_of_participants >= lotto_game_data.winner_limit{
            msg!("Lotto game has reached its participant limit");
            return Err(ParticipantLimitReachedError.into());
          }

          // diziyi Pubkey'e donustur
          let user_pubkey = Pubkey::new_from_array(ticket_data.user_address);
          
          let transfer_instruction = system_instruction::transfer(
            payer.key,
            &user_pubkey, 
            lotto_game_data.prize_amount
          );
          
          invoke(
            &transfer_instruction,
            &[payer.clone(), ticket.clone()],
        )?;
        
        // boleti sil
        ticket_data.gameid = 0;
        ticket_data.user_address = Pubkey::default().to_bytes() ;
        ticket_data.participant_numbers = [0;5];
   
        lotto_game_data.serialize(&mut &mut lotto_game.try_borrow_mut_data()?[..])?;
        ticket_data.serialize(&mut &mut ticket.try_borrow_mut_data()?[..])?;

        Ok(())
      }

      // Otorite olustur
     pub fn create_authority (
      accounts: &[AccountInfo],
      program_id: &Pubkey,
    ) -> ProgramResult{

      let account_info_iter = &mut accounts.iter();
      let payer = next_account_info(account_info_iter)?;
      let authority = next_account_info(account_info_iter)?;

      let(authority_address, bump) = Pubkey::find_program_address(&[b"authority"], program_id);

      let rent = Rent:: default();
      let lamports = rent.minimum_balance(32);

      invoke_signed ( 
        &system_instruction::create_account(payer.key, &authority_address, lamports, 32, program_id),
        &[authority.clone(),payer.clone()],
        &[
          &[b"authority" ,&[bump]]]
       )?;
       
       let authority_data = Authority{ 
        authority_accounts:payer.key.to_bytes(), 
        };

       authority_data.serialize(&mut &mut authority.data.borrow_mut()[..])?;
       
      Ok(())
    }


    
}