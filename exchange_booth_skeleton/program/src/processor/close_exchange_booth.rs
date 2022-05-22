use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
    account_info::{next_account_info},
    program::{invoke, invoke_signed},
    program_pack::Pack,
};

use crate::{
    error::ExchangeBoothError,
    state::ExchangeBooth,
};

use spl_token::{
    state::{Account as SPLTokenAccount, Mint},
    instruction as token_instruction,
 };

use borsh::{BorshDeserialize, BorshSerialize};


pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    // ???
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let cur_exchange_booth_account = next_account_info(accounts_iter)?;
    let admin_signer_account = next_account_info(accounts_iter)?;
    let vault1 = next_account_info(accounts_iter)?;
    let vault2 = next_account_info(accounts_iter)?;
    let mint1 = next_account_info(accounts_iter)?;
    let mint2 = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let exchange_booth_obj = ExchangeBooth::try_from_slice(&cur_exchange_booth_account.data.borrow())?;

    let (exchange_booth_key, exchange_booth_bump) = Pubkey::find_program_address(
        &[
            b"exchange_booth",
            admin_signer_account.key.as_ref()
        ],
        program_id,
    );

    assert_eq!(&exchange_booth_key, cur_exchange_booth_account.key);

    let (vault1_key, vault1_bump) = Pubkey::find_program_address(
        &[
           b"vault_from_mint",
           admin_signer_account.key.as_ref(),
           cur_exchange_booth_account.key.as_ref(),
           mint1.key.as_ref()
       ],
       program_id,
    );
    
    assert_eq!(&vault1_key, vault1.key);

    let (vault2_key, vault2_bump) = Pubkey::find_program_address(
        &[
           b"vault_from_mint",
           admin_signer_account.key.as_ref(),
           cur_exchange_booth_account.key.as_ref(),
           mint2.key.as_ref()
       ],
       program_id,
    );
    
    assert_eq!(&vault2_key, vault2.key);

    let vault1_acnt_obj = (spl_token::state::Account::unpack_from_slice(&vault1.data.borrow())?);
    let vault2_acnt_obj = (spl_token::state::Account::unpack_from_slice(&vault2.data.borrow())?);

    msg!("vault 1 amount: {:?}", vault1_acnt_obj.amount);
    msg!("failed key: {:?}", exchange_booth_key);
    msg!("failed key: {:?}", admin_signer_account.key);
    msg!("failed key: {:?}", mint1.key);
    msg!("failed key: {:?}", mint2.key);

    invoke_signed(
        &token_instruction::burn(
            &token_program.key,
            &vault1.key,
            &mint1.key,
            &admin_signer_account.key,
            &[],
            vault1_acnt_obj.amount
        )?,
        &[vault1.clone(), mint1.clone(), admin_signer_account.clone(), token_program.clone(), system_program.clone(), rent.clone()],
        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint1.key.as_ref(), &[vault1_bump]]]
    )?;

    invoke_signed(
        &token_instruction::burn(
            &token_program.key,
            &vault2.key,
            &mint2.key,
            &admin_signer_account.key,
            &[],
            vault2_acnt_obj.amount
        )?,
        &[vault2.clone(), mint2.clone(), admin_signer_account.clone(), token_program.clone(), system_program.clone(), rent.clone()],
        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint2.key.as_ref(), &[vault2_bump]]]
    )?;

    invoke_signed(
        &token_instruction::close_account(
            &token_program.key,
            &vault1.key,
            &admin_signer_account.key, 
            &admin_signer_account.key, 
            &[]
        )?,
        &[vault1.clone(), mint1.clone(), admin_signer_account.clone(), token_program.clone(), system_program.clone(), rent.clone()],
        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint1.key.as_ref(), &[vault1_bump]]]
    )?;

    invoke_signed(
        &token_instruction::close_account(
            &token_program.key,
            &vault2.key,
            &admin_signer_account.key, 
            &admin_signer_account.key, 
            &[]
        )?,
        &[vault2.clone(), mint2.clone(), admin_signer_account.clone(), token_program.clone(), system_program.clone(), rent.clone()],
        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint2.key.as_ref(), &[vault2_bump]]]
    )?;

    Ok(())
}