use solana_program::{
    account_info::next_account_info
};

use crate::{
    error::ExchangeBoothError,
    state::{ExchangeBooth, TokenAccount}
};

use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    withdrawal_amount: f64,
    // ???
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let cur_exchange_booth_account = next_account_info(accounts_iter)?;
    let admin_signer_account = next_account_info(accounts_iter)?;
    let account_to_wthdrw_from = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let vault_account = next_account_info(accounts_iter)?;

    let (exchange_booth_key, exchange_booth_bump) = Pubkey::find_program_address(
        &[
            b"exchange_booth",
            admin_signer_account.key.as_ref()
        ],
        program_id,
    );

    let exchange_booth_data = ExchangeBooth::try_from_slice(&cur_exchange_booth_account.data.borrow_mut())?;
    
    assert_eq!(*admin_signer_account.key, exchange_booth_data.Admin); //verifies that our signing account is the correct signer for the exchange booth account we have access to.
    assert_eq!(*cur_exchange_booth_account.key, exchange_booth_key); //verifies that we are accessing the right exchange booth

    assert!(*mint.key == exchange_booth_data.TokenMint1 || *mint.key == exchange_booth_data.TokenMint2);

    let (vault_key, vault_bump) = Pubkey::find_program_address(
        &[
            b"vault_from_mint",
            admin_signer_account.key.as_ref(),
            cur_exchange_booth_account.key.as_ref(),
            mint.key.as_ref()
        ],
        program_id,
    );

    assert_eq!(*vault_account.key, vault_key);
    
    let mut vault_account_data = TokenAccount::try_from_slice(*vault_account.data.borrow())?;
    let mut withdrawal_data = TokenAccount::try_from_slice(*account_to_wthdrw_from.data.borrow())?;
    
    msg!("initial vault: {}", vault_account_data.amount);
    msg!("initial wallet: {}", withdrawal_data.amount);

    if vault_account_data.amount >= withdrawal_amount {
        vault_account_data.amount -= withdrawal_amount;
        withdrawal_data.amount += withdrawal_amount;
        msg!("final vault: {}", vault_account_data.amount);
        msg!("final wallet: {}", withdrawal_data.amount);
        msg!("successful transaction");
    } else {
        msg!("invalid move");
    }

    vault_account_data.serialize(&mut *vault_account.data.borrow_mut())?;
    withdrawal_data.serialize(&mut *account_to_wthdrw_from.data.borrow_mut())?;

    Ok(())
}
