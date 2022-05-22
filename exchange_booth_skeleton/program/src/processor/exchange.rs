use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
    program::{invoke_signed, invoke}
};

use crate::{
    error::ExchangeBoothError,
    state::{ExchangeBooth, OracleModel}
};

use spl_token::{
    state::{Account as SPLTokenAccount, Mint},
    instruction as token_instruction,
 };

use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    borsh::try_from_slice_unchecked,
    program_pack::Pack,
};


pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    num_tokens_to_deposit: f64,
    // ???
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let cur_exchange_booth_account = next_account_info(accounts_iter)?;
    let admin_signer_account = next_account_info(accounts_iter)?;
    let wallet_deposit = next_account_info(accounts_iter)?;
    let wallet_withdraw = next_account_info(accounts_iter)?;
    let vault_deposit = next_account_info(accounts_iter)?;
    let vault_withdraw = next_account_info(accounts_iter)?;
    let wallet_mint_deposit = next_account_info(accounts_iter)?;
    let wallet_mint_withdraw = next_account_info(accounts_iter)?;
    let vault_mint_deposit = next_account_info(accounts_iter)?;
    let vault_mint_withdraw = next_account_info(accounts_iter)?;
    let oracle_accnt = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let (exchange_booth_key, exchange_booth_bump) = Pubkey::find_program_address(
        &[
            b"exchange_booth",
            admin_signer_account.key.as_ref()
        ],
        program_id,
    );

    assert_eq!(cur_exchange_booth_account.key, &exchange_booth_key);

    let mut exchange_booth_object = ExchangeBooth::try_from_slice(&cur_exchange_booth_account.data.borrow())?;

    let check_bool = if *vault_mint_deposit.key == exchange_booth_object.TokenMint1 {
        "Mint 1"
    } else if *vault_mint_deposit.key == exchange_booth_object.TokenMint2 {
        "Mint 2"
    } else {
        "Bad"
    };

    msg!("checking type of token: {:?}", check_bool);

    assert_ne!(check_bool, "Bad");

    let (vault_key, vault_bump) = Pubkey::find_program_address(
        &[
           b"vault_from_mint",
           admin_signer_account.key.as_ref(),
           cur_exchange_booth_account.key.as_ref(),
           vault_mint_deposit.key.as_ref()
       ],
       program_id,
    );

    assert_eq!(&vault_key, vault_deposit.key);

    let (oracle_key, oracle_bump) = Pubkey::find_program_address(
        &[
            b"oracle",
            exchange_booth_object.TokenMint1.as_ref(),
            exchange_booth_object.TokenMint2.as_ref(),
            admin_signer_account.key.as_ref()
        ],
        program_id
    );
  
    assert_eq!(*oracle_accnt.key, oracle_key);

    let oracle_data = OracleModel::try_from_slice(&oracle_accnt.data.borrow())?;
    // oracle_data.Mint1ExchangeRate; equal to 1
    // oracle_data.Mint2ExchangeRate; equal to 1.254

    //if we are exchanging mint 1 for mint 2, we want to multiply by Mint2ExchangeRate/Mint1ExchangeRate
    //if we are exchanging mint2 for mint1, we want to multiply by Mint1ExchangeRate/Mint2ExchangeRate

    let mut wallet_subtraction = num_tokens_to_deposit;
    let mut vault_subtraction = 0.0;

    msg!("right before token instructions");

    if check_bool == "Mint 1" {
        //vault is mint1, so exchanging mint 2 for mint 1
        vault_subtraction = num_tokens_to_deposit * (oracle_data.Mint1ExchangeRate/oracle_data.Mint2ExchangeRate);
    } else {
        vault_subtraction = num_tokens_to_deposit * (oracle_data.Mint2ExchangeRate/oracle_data.Mint1ExchangeRate);
    }

    msg!("{:?}", token_program.key);

    invoke_signed(
        &token_instruction::transfer(
            &token_program.key, 
            &wallet_deposit.key,
            &vault_deposit.key, 
            &admin_signer_account.key, 
            &[],
            (wallet_subtraction*1000000000.0) as u64
        )?,
        &[vault_deposit.clone(), wallet_mint_deposit.clone(), admin_signer_account.clone(), wallet_deposit.clone(), token_program.clone(), system_program.clone(), rent.clone()],
        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), wallet_mint_deposit.key.as_ref(), &[vault_bump]]]
    )?;

    msg!("before final transfer call: {:?}", token_program.key);

    invoke_signed(
        &token_instruction::transfer(
            &token_program.key, 
            &vault_withdraw.key,
            &wallet_withdraw.key, 
            &admin_signer_account.key, 
            &[],
            (vault_subtraction*1000000000.0) as u64
        )?,
        &[vault_withdraw.clone(), wallet_mint_withdraw.clone(), admin_signer_account.clone(), wallet_withdraw.clone(), token_program.clone(), system_program.clone(), rent.clone()],
        &[&[]]
    )?;

    Ok(())
}