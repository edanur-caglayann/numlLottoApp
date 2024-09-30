use core::borrow;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{ 
    account_info::{next_account_info, AccountInfo}, clock, config, entrypoint::ProgramResult, lamports, msg, program::{invoke, invoke_signed}, program_error::ProgramError, pubkey::{self, Pubkey, PubkeyError}, rent::Rent, system_instruction::{self}, system_program, sysvar::Sysvar
    };
    use crate::{instruction::RNGProgramInstruction, state::{ DrawData, GameCount, LottoGame, Ticket}};
    use crate::error::RNGProgramError::{InvalidInstruction, ArithmeticErr, NotActiveErr, AuthorityError, GameIdMismatchError, NoPrizeError, OwnershipError, ParticipantLimitReachedError, AlreadyActiveError, TicketFeeNotPaidError,YouNotWinnerError,UpdateError,InsufficientFundsError};
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
          RNGProgramInstruction:: Ticket{participant_numbers} => {
              Self::ticket( accounts,_program_id,participant_numbers)
             },  
          RNGProgramInstruction:: Draw{prize_amount,winning_numbers }  => {
            Self::draw( accounts,_program_id,prize_amount,winning_numbers)
             },
          RNGProgramInstruction:: ClaimPrize => {
              Self::claim_prize( accounts,_program_id)
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


        let rent = Rent:: default();
        let game_rent = rent.minimum_balance(33);
        
        let(lotto_game_pda, bump) = Pubkey::find_program_address(&[b"lotto_game",count_data.game_count.to_string().as_ref()], program_id);

        invoke_signed(
          &system_instruction::create_account(payer.key, &lotto_game_pda, game_rent, 33, program_id), 
          &[lotto_game.clone(),payer.clone()], 
          &[
            &[b"lotto_game",count_data.game_count.to_string().as_ref(),&[bump]]
          ]
        )?;

        let ticket_price: u64 = 1_000_000_000;

        let lotto_game_info = LottoGame {
            gameid: count_data.game_count,
            number_of_participants: 0,
            winning_numbers:[0; 5],
            number_of_winner: 0,
            prize_pool: 0,
            prize_amount: 15,
            is_active: 1, // oyunu baslattik``
            ticket_money: ticket_price// 1 sol bilet bedeli
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
    
      // katilim fonks-bilet olsuturma
      pub fn ticket (
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        participant_numbers: [u8; 5], // kullanici num par. olarak aliniyor. kullanici kendi belirliyor
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
     
       // prize pool guncelleme
        lotto_game_data.prize_pool = lotto_game_data.prize_pool.checked_add(lotto_game_data.ticket_money).ok_or(TicketFeeNotPaidError)?; 

        // katilimci sayisini arttiralim
        lotto_game_data.number_of_participants = lotto_game_data.number_of_participants.checked_add(1).ok_or(ArithmeticErr)?; 

        let (ticket_pda, bump) = Pubkey::find_program_address(
          &[b"ticket", lotto_game_data.gameid.to_string().as_ref(), b"gameNo", lotto_game_data.number_of_participants.to_string().as_ref()],
          program_id,
       );

      let rent = Rent:: default();
      let ticket_rent = rent.minimum_balance(38) ;

      let ticket_total_lamp = ticket_rent.checked_add(lotto_game_data.prize_amount).ok_or(InvalidInstruction)?;

      invoke_signed(
        &system_instruction::create_account(payer.key, &ticket_pda, ticket_total_lamp, 38, program_id), 
        &[ticket.clone(),payer.clone()], 
        &[
          &[b"ticket", lotto_game_data.gameid.to_string().as_ref(), b"gameNo",lotto_game_data.number_of_participants.to_string().as_ref(),&[bump]]
        ]
      )?;

        let ticket_info = Ticket {
            gameid: lotto_game_data.gameid,
            user_address:payer.key.to_bytes(),
            participant_numbers,
        };
        

        **ticket.try_borrow_mut_lamports()? -= lotto_game_data.prize_amount;
        **lotto_game.try_borrow_mut_lamports()? += lotto_game_data.prize_amount;

        ticket_info.serialize(&mut &mut ticket.try_borrow_mut_data()?[..])?;
        lotto_game_data.serialize(&mut &mut lotto_game.try_borrow_mut_data()?[..])?;
        Ok(())
      }
    
      // cekilis
      pub fn draw (
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        prize_amount: u64,
        winning_numbers: [u8; 5],
      ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let lotto_game   = next_account_info(account_info_iter)?;
        let ticket =  next_account_info(account_info_iter)?;

           //payer imzalayici mi
        if !payer.is_signer {
          return Err(ProgramError::MissingRequiredSignature);
           }

        if lotto_game.owner!= program_id {
          msg!("not a program for authority");
          return Err(OwnershipError.into());
          }

        let mut lotto_game_data = LottoGame::try_from_slice(&lotto_game.data.borrow())?; 
        let mut ticket_data = Ticket::try_from_slice(&ticket.data.borrow())?; 
        
       //odul aktif degilse odul verilemez
        if lotto_game_data.is_active == 0 {
          msg!("The game is not active");
          return Err(ArithmeticErr.into());
        }
        
        if ticket_data.user_address != payer.key.to_bytes() {
          msg!("This ticket does not belong to the payer.");
         return Err(OwnershipError.into());
        }
        
        if lotto_game_data.prize_amount == 0 {
          msg!("The prize amount of the lotto game is zero.");
          return Err(ProgramError::InvalidAccountData);
        }

        if ticket_data.gameid != lotto_game_data.gameid{
          msg!("Ticket does not match this lotto game.");
          return Err(GameIdMismatchError.into());
        }

        if ticket_data.participant_numbers != winning_numbers {
          msg!("Not among the winning numbers");
          return Err(YouNotWinnerError.into());
        }

        // odulu guncelleme
        lotto_game_data.prize_amount = prize_amount;
        lotto_game_data.winning_numbers = winning_numbers; 
        
        //cekilis olunca o oyun icin olan kullanici sayisi sifirlanir
        lotto_game_data.number_of_participants = 0;
       
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
        let lotto_game   = next_account_info(account_info_iter)?;
        let ticket =  next_account_info(account_info_iter)?;

           // otorite imzalayici mi
           if !payer.is_signer {
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
          let ticket_data = Ticket::try_from_slice(&ticket.data.borrow())?; 

            // ticket gercekten payera mi ait kontrol et
            if ticket_data.user_address != payer.key.to_bytes() {
              msg!("This ticket does not belong to the payer.");
             return Err(OwnershipError.into());
            }

            if ticket_data.gameid != lotto_game_data.gameid{
              msg!("Ticket does not match this lotto game.");
              return Err(GameIdMismatchError.into());
            }
            
            if lotto_game_data.is_active == 0 {
              msg!("The game is not active");
              return Err(ArithmeticErr.into());
            }   

            // Prize pool'dan ödül miktarını kontrol et ve ödülü dağıt
             if lotto_game_data.prize_pool < lotto_game_data.prize_amount {
              msg!("Not enough funds in the prize pool");
               return Err(InsufficientFundsError.into());
            }

            // Transfer yapilan account programa ait oldugu icin tarnsfer yapmak yerine lamports'unu modifiye edebiliriz
            **lotto_game.try_borrow_mut_lamports()? -= lotto_game_data.prize_amount;
            **payer.try_borrow_mut_lamports()? += lotto_game_data.prize_amount;

            // ticket'i sildik. ticketin rentini kim olusturduysa ona geri gondererek silebilriiz
            let ticket_value =  **ticket.try_borrow_lamports()? ;
            
            **ticket.try_borrow_mut_lamports()? -= ticket_value;
            **payer.try_borrow_mut_lamports()? += ticket_value;
            
             // prize poolda kalan para
             lotto_game_data.prize_pool = lotto_game_data.prize_pool.checked_sub(lotto_game_data.prize_amount).ok_or(UpdateError)?;

            lotto_game_data.serialize(&mut &mut lotto_game.try_borrow_mut_data()?[..])?;

        Ok(())
      }

}