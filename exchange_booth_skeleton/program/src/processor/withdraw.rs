use solana_program::{
    account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
    program::{invoke_signed}
};

use crate::{
    error::ExchangeBoothError,
    state::ExchangeBooth,
};

use borsh::{BorshDeserialize, BorshSerialize};

use spl_token::{
    state::{Account as SPLTokenAccount, Mint},
    instruction as token_instruction,
 };

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    withdrawal_amount: f64
    // ???
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let cur_exchange_booth_account = next_account_info(accounts_iter)?;
    let admin_signer_account = next_account_info(accounts_iter)?;
    let user_wallet = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let vault = next_account_info(accounts_iter)?;
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
  
    assert_eq!(&exchange_booth_key, cur_exchange_booth_account.key);

    let exchange_booth_object = ExchangeBooth::try_from_slice(&cur_exchange_booth_account.data.borrow())?;

    msg!("{:?}", mint.key);
    msg!("{:?} \t\t {:?} \t\t {:?}", mint.key, exchange_booth_object.TokenMint1, exchange_booth_object.TokenMint2);

    let check_bool = if *mint.key == exchange_booth_object.TokenMint1 {
        "Equal to Mint 1"
    } else if *mint.key == exchange_booth_object.TokenMint2 {
        "Equal to Mint 2"
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
           mint.key.as_ref()
       ],
       program_id,
    );
 
    assert_eq!(&vault_key, vault.key);

    msg!("starting the token transfer process");

    invoke_signed(
        &token_instruction::transfer(
            &token_program.key, 
            &vault.key,
            &user_wallet.key, 
            &admin_signer_account.key, 
            &[], 
            (withdrawal_amount*1000000000.0) as u64
        )?,
        &[vault.clone(), mint.clone(), admin_signer_account.clone(), user_wallet.clone(), token_program.clone(), system_program.clone(), rent.clone()],
        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint.key.as_ref(), &[vault_bump]]]
    )?;

    Ok(())
}
