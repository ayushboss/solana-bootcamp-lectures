use std::mem;
 
use solana_program::{
   account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, program_error::ProgramError,
   pubkey::Pubkey,
   system_instruction,
   sysvar::{rent::Rent, Sysvar},
   program::{invoke_signed, invoke}
};
 
use spl_token::{
   state::{Account as SPLTokenAccount, Mint},
   instruction as token_instruction,
};
 
use crate::{
   state::{ExchangeBooth, OracleModel, AccountTag},
   state::TokenAccount,
};
 
use borsh::{BorshDeserialize, BorshSerialize};
 
pub fn process(
   program_id: &Pubkey,
   accounts: &[AccountInfo],
   tokenExchange1: f64,
   tokenExchange2: f64,
   // ???
) -> ProgramResult {
 
   msg!("token exchange 1: {}", tokenExchange1);
   msg!("token exchange 2: {}", tokenExchange2);
 
   //ExchangeBooth
   //  TokenMint1
   //  TokenMint2
   //  Oracle
   //  Admin (signer)
 
   let accounts_iter = &mut accounts.iter();
   let cur_exchange_booth_account = next_account_info(accounts_iter)?;
   let admin_signer_account = next_account_info(accounts_iter)?;
   let spl_token_admin = next_account_info(accounts_iter)?;
   let mint1 = next_account_info(accounts_iter)?;
   let mint2 = next_account_info(accounts_iter)?;
   let oracle_accnt = next_account_info(accounts_iter)?;
   let vault1 = next_account_info(accounts_iter)?;
   let vault2 = next_account_info(accounts_iter)?;
   let admin_controlled_user_account_mint1 = next_account_info(accounts_iter)?;
   let admin_controlled_user_account_mint2 = next_account_info(accounts_iter)?;
   let rent_sysvar_info = next_account_info(accounts_iter)?;
   let system_program = next_account_info(accounts_iter)?;

   let rent = &Rent::from_account_info(rent_sysvar_info)?;

   // initialize the vaults as PDAs of this authority program
   // use seeds of this program and specific mint
  
   // token_instruction::initialize_mint(decimals, pubkey, freezeauthority (multiple signers))
 
   let (exchange_booth_key, exchange_booth_bump) = Pubkey::find_program_address(
       &[
           b"exchange_booth",
           admin_signer_account.key.as_ref()
       ],
       program_id,
   );
  
   let (oracle_key, oracle_bump) = Pubkey::find_program_address(
       &[
           b"oracle",
           mint1.key.as_ref(),
           mint2.key.as_ref(),
           admin_signer_account.key.as_ref()
       ],
       program_id
   );
 
   assert_eq!(*cur_exchange_booth_account.key, exchange_booth_key);
   assert_eq!(*oracle_accnt.key, oracle_key);
   assert_eq!(spl_token::id(), *spl_token_admin.key);

    // SystemProgram.createAccount({
    //   fromPubkey: feePayer,
    //   newAccountPubkey: mint1.publicKey,
    //   lamports: await connection.getMinimumBalanceForRentExemption(82),
    //   space: 82,
    //   programId: TOKEN_PROGRAM_ID
    // })

    // system_instruction::create_account(
    //     from_pubkey: &Pubkey,
    //     to_pubkey: &Pubkey, 
    //     lamports: u64, 
    //     space: u64, 
    //     owner: &Pubkey
    // )
   msg!("checkpoint 3");
   msg!("{}",&spl_token::id());
   invoke(
       &token_instruction::initialize_mint(
           spl_token_admin.key,
           mint1.key,
           admin_signer_account.key,
           Some(spl_token_admin.key),
           9)?,
        &[
            mint1.clone(),
            rent_sysvar_info.clone(),
            spl_token_admin.clone(),
            admin_signer_account.clone()
        ],
    )?;

   msg!("checkpoint 4");
 
    invoke(
        &token_instruction::initialize_mint(
            spl_token_admin.key,
            mint2.key,
            admin_signer_account.key,
            Some(spl_token_admin.key),
            9
        )?,
        &[
            mint2.clone(),
            rent_sysvar_info.clone(),
            spl_token_admin.clone(),
            admin_signer_account.clone()
        ],
    )?;
 
    invoke_signed(
        &system_instruction::create_account(
            admin_signer_account.key,
            &exchange_booth_key,
            Rent::get()?.minimum_balance(128),
            128,
            program_id
        ),
        &[admin_signer_account.clone(), cur_exchange_booth_account.clone(), system_program.clone()],
        &[&[b"exchange_booth", admin_signer_account.key.as_ref(), &[exchange_booth_bump]]]
    )?;

    invoke_signed(
        &system_instruction::create_account(
            admin_signer_account.key,
            &oracle_accnt.key,
            Rent::get()?.minimum_balance(80),
            80,
            program_id
        ),
        &[admin_signer_account.clone(), oracle_accnt.clone(), system_program.clone()],
        &[&[b"oracle", mint1.key.as_ref(), mint2.key.as_ref(), admin_signer_account.key.as_ref(), &[oracle_bump]]]
    )?;

   let oracle_fill_obj = OracleModel {
       TokenMint1: *mint1.key,
       TokenMint2: *mint2.key,
       Mint1ExchangeRate: tokenExchange1,
       Mint2ExchangeRate:  tokenExchange2,
   };
 
   oracle_fill_obj.serialize(&mut *oracle_accnt.data.borrow_mut())?;
 
   let exchange_booth_obj = ExchangeBooth {
       TokenMint1: *mint1.key,
       TokenMint2: *mint2.key,
       OracleSeed: *oracle_accnt.key,
       Admin: *admin_signer_account.key,
   };
      
   exchange_booth_obj.serialize(&mut *cur_exchange_booth_account.data.borrow_mut())?;
 
   //gonna create the vaults now
   let (vault_mint_1_key, vault_mint_1_bump) = Pubkey::find_program_address(
       &[
           b"vault_from_mint",
           admin_signer_account.key.as_ref(),
           cur_exchange_booth_account.key.as_ref(),
           mint1.key.as_ref()
       ],
       program_id,
   );
 
   let (vault_mint_2_key, vault_mint_2_bump) = Pubkey::find_program_address(
       &[
           b"vault_from_mint",
           admin_signer_account.key.as_ref(),
           cur_exchange_booth_account.key.as_ref(),
           mint2.key.as_ref()
       ],
       program_id,
   );
 
   assert_eq!(*vault1.key, vault_mint_1_key);
   assert_eq!(*vault2.key, vault_mint_2_key);

   invoke_signed(
        &system_instruction::create_account(
            admin_signer_account.key,
            &vault1.key,
            Rent::get()?.minimum_balance(165),
            165,
            spl_token_admin.key
        ),
        &[admin_signer_account.clone(), vault1.clone(), system_program.clone()],
        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint1.key.as_ref(), &[vault_mint_1_bump]]]
    )?;

    invoke_signed(
        &system_instruction::create_account(
            admin_signer_account.key,
            &vault2.key,
            Rent::get()?.minimum_balance(165),
            165,
            spl_token_admin.key
        ),
        &[admin_signer_account.clone(), vault2.clone(), system_program.clone()],
        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint2.key.as_ref(), &[vault_mint_2_bump]]]
    )?;

    msg!("initializing the vaults");

   invoke_signed(
       &token_instruction::initialize_account(
           spl_token_admin.key,
           vault1.key, 
           mint1.key, 
           admin_signer_account.key
        )?,
        &[vault1.clone(), mint1.clone(), admin_signer_account.clone(), spl_token_admin.clone(), system_program.clone(), rent_sysvar_info.clone()],
        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint1.key.as_ref(), &[vault_mint_1_bump]]]
   )?;

   msg!("initializing vault 2");

   invoke_signed(
        &token_instruction::initialize_account(
            spl_token_admin.key,
            vault2.key, 
            mint2.key, 
            admin_signer_account.key
        )?,
        &[vault2.clone(), mint2.clone(), admin_signer_account.clone(), spl_token_admin.clone(), system_program.clone(), rent_sysvar_info.clone()],
        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint2.key.as_ref(), &[vault_mint_2_bump]]]
    )?;
    
    msg!("created token accounts for each of the vaults, now am going to mint several tokens to each of the accounts");

    invoke_signed(
        &token_instruction::mint_to(
            &spl_token_admin.key, 
            &mint1.key, 
            &vault1.key, 
            admin_signer_account.key,
            &[&admin_signer_account.key], 
            1000000000,
        )?,
        &[vault1.clone(), mint1.clone(), admin_signer_account.clone(), spl_token_admin.clone(), system_program.clone(), rent_sysvar_info.clone()],
        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint1.key.as_ref(), &[vault_mint_1_bump]]]
    )?;

    invoke_signed(
        &token_instruction::mint_to(
            &spl_token_admin.key, 
            &mint2.key, 
            &vault2.key, 
            admin_signer_account.key, 
            &[&admin_signer_account.key], 
            1000000000,
        )?,
        &[vault2.clone(), mint2.clone(), admin_signer_account.clone(), spl_token_admin.clone(), system_program.clone(), rent_sysvar_info.clone()],
        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint2.key.as_ref(), &[vault_mint_2_bump]]]
    )?;


    msg!("vaults are initialized, now need to make the personal token wallets");

    invoke(
        &token_instruction::initialize_account(
            spl_token_admin.key,
            admin_controlled_user_account_mint1.key, 
            mint1.key, 
            admin_signer_account.key
         )?,
         &[admin_controlled_user_account_mint1.clone(), mint1.clone(), admin_signer_account.clone(), spl_token_admin.clone(), system_program.clone(), rent_sysvar_info.clone()]
    )?;

    invoke(
        &token_instruction::initialize_account(
            spl_token_admin.key,
            admin_controlled_user_account_mint2.key, 
            mint2.key, 
            admin_signer_account.key
         )?,
         &[admin_controlled_user_account_mint2.clone(), mint2.clone(), admin_signer_account.clone(), spl_token_admin.clone(), system_program.clone(), rent_sysvar_info.clone()]
    )?;

    invoke(
        &token_instruction::mint_to(
            &spl_token_admin.key, 
            &mint1.key, 
            &admin_controlled_user_account_mint1.key, 
            admin_signer_account.key,
            &[&admin_signer_account.key], 
            1000000000,
        )?,
        &[admin_controlled_user_account_mint1.clone(), mint1.clone(), admin_signer_account.clone(), spl_token_admin.clone(), system_program.clone(), rent_sysvar_info.clone()]
    )?;

    invoke(
        &token_instruction::mint_to(
            &spl_token_admin.key, 
            &mint2.key, 
            &admin_controlled_user_account_mint2.key, 
            admin_signer_account.key, 
            &[&admin_signer_account.key], 
            1000000000,
        )?,
        &[admin_controlled_user_account_mint2.clone(), mint2.clone(), admin_signer_account.clone(), spl_token_admin.clone(), system_program.clone(), rent_sysvar_info.clone()]
    )?;
    
//    invoke_signed(
//        &system_instruction::create_account(
//            admin_signer_account.key,
//            &vault1.key,
//            Rent::get()?.minimum_balance(165),
//            165,
//            program_id
//        ),
//        &[admin_signer_account.clone(), vault1.clone(), system_program.clone()],
//        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint1.key.as_ref(), &[vault_mint_1_bump]]]
//    )?;
 
//    invoke_signed(
//        &system_instruction::create_account(
//            admin_signer_account.key,
//            &vault2.key,
//            Rent::get()?.minimum_balance(165),
//            165,
//            program_id
//        ),
//        &[admin_signer_account.clone(), vault2.clone(), system_program.clone()],
//        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint2.key.as_ref(), &[vault_mint_2_bump]]]
//    )?;
 
//    invoke(
//        &token_instruction::initialize_account(
//            program_id,
//            &vault1.key,
//            &mint1.key,
//            admin_signer_account.key,
//        )?,
//        &[admin_signer_account.clone(), vault1.clone(), system_program.clone()]
//    )?;
 
//    invoke(
//        &token_instruction::initialize_account(
//            program_id,
//            &vault2.key,
//            &mint2.key,
//            admin_signer_account.key,
//        )?,
//        &[admin_signer_account.clone(), vault2.clone(), system_program.clone()]
//    )?;
 
//    invoke_signed(
//        &token_instruction::mint_to(
//            &program_id,
//            &mint1.key,
//            &vault1.key,
//            &admin_signer_account.key,
//            &[&admin_signer_account.key],
//            10
//        )?,
//        &[mint1.clone(), vault1.clone(), admin_signer_account.clone(), system_program.clone()],
//        &[]
//    )?;
 
//    invoke_signed(
//        &token_instruction::mint_to(
//            &program_id,
//            &mint2.key,
//            &vault2.key,
//            &admin_signer_account.key,
//            &[&admin_signer_account.key],
//            10
//        )?,
//        &[mint2.clone(), vault2.clone(), admin_signer_account.clone(), system_program.clone()],
//        &[]
//    )?;
 
//    // let empty_token_account_vault_1 = TokenAccount {
//    //     tag: AccountTag::TokenAccount2,
//    //     amount: 0.0,
//    //     mint: *mint1.key,
//    //     owner: *admin_signer_account.key,
//    // };
 
//    // let empty_token_account_vault_2 = TokenAccount {
//    //     tag: AccountTag::TokenAccount2,
//    //     amount: 0.0,
//    //     mint: *mint2.key,
//    //     owner: *admin_signer_account.key,
//    // };
 
//    // empty_token_account_vault_1.serialize(&mut *vault1.data.borrow_mut())?;
//    // empty_token_account_vault_2.serialize(&mut *vault2.data.borrow_mut())?;
  
//    invoke(
//        &token_instruction::initialize_account(
//            program_id,
//            &admin_controlled_user_account_mint1.key,
//            &mint1.key,
//            admin_signer_account.key,
//        )?,
//        &[admin_signer_account.clone(), admin_controlled_user_account_mint1.clone(), system_program.clone()]
//    )?;
 
//    invoke(
//        &token_instruction::initialize_account(
//            program_id,
//            &admin_controlled_user_account_mint2.key,
//            &mint2.key,
//            admin_signer_account.key,
//        )?,
//        &[admin_signer_account.clone(), admin_controlled_user_account_mint2.clone(), system_program.clone()]
//    )?;
 
 
//    // let mint1_user_token_account = TokenAccount {
//    //     tag: AccountTag::TokenAccount2,
//    //     amount: 2.000,
//    //     mint: *mint1.key,
//    //     owner: *admin_signer_account.key,
//    // };
  
//    // let mint2_user_token_account = TokenAccount {
//    //     tag: AccountTag::TokenAccount2,
//    //     amount: 2.000,
//    //     mint: *mint2.key,
//    //     owner: *admin_signer_account.key,
//    // };
 
//    // mint1_user_token_account.serialize(&mut *admin_controlled_user_account_mint1.data.borrow_mut())?;
//    // mint2_user_token_account.serialize(&mut *admin_controlled_user_account_mint2.data.borrow_mut())?;
 
   Ok(())
}
 

