use solana_program::{
};

use crate::{
    error::ExchangeBoothError,
    state::{ExchangeBooth, TokenAccount, OracleModel}
};

use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount_to_convert: u64,
    amount_to_convert_scale: u64,
    // ???
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let cur_exchange_booth_account = next_account_info(accounts_iter)?;
    let admin_signer_account = next_account_info(accounts_iter)?;
    let wallet_original_mint = next_account_info(accounts_iter)?;
    let wallet_converted_mint = next_account_info(accounts_iter)?;
    let vault1 = next_account_info(accounts_iter)?;
    let vault2 = next_account_info(accounts_iter)?;
    let oracle = next_account_info(accounts_iter)?;
    
    let exchange_booth_object = ExchangeBooth::try_from_slice(&mut cur_exchange_booth_account.data.borrow())?;
    let mut wallet_original_mint_object = TokenAccount::try_from_slice(&mut wallet_original_mint.data.borrow())?;
    let mut wallet_converted_mint_object = TokenAccount::try_from_slice(&mut wallet_converted_mint.data.borrow())?;

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
            exchange_booth_object.TokenMint1.as_ref(),
            exchange_booth_object.TokenMint2.as_ref(), 
            admin_signer_account.key.as_ref()
        ],
        program_id
    );

    assert_eq!(*cur_exchange_booth_account.key, exchange_booth_key);
    assert_eq!(*oracle.key, oracle_key);
    
    let oracle_data = OracleModel::try_from_slice(&mut oracle.data.borrow())?;

    assert_eq!(oracle_data.TokenMint1, exchange_booth_object.TokenMint1);
    assert_eq!(oracle_data.TokenMint2, exchange_booth_object.TokenMint2);

    // pub struct OracleModel {
    //     pub TokenMint1: Pubkey,
    //     pub TokenMint2: Pubkey,
    //     pub Mint1ExchangeRate: f64,
    //     pub Mint2ExchangeRate: f64,
    // }

    let rate1 = oracle_data.Mint1ExchangeRate; 
    let rate2 = oracle_data.Mint2ExchangeRate;
    // rate1 == rate2

    let mut vault1_object = TokenAccount::try_from_slice(&mut vault1.data.borrow())?;
    let mut vault2_object = TokenAccount::try_from_slice(&mut vault2.data.borrow())?;

    // if wallet_original_mint_object.mint == exchange_booth_object.TokenMint1 {
    //     let converted_value = amount_to_convert * (rate2/rate1);

    //     wallet_original_mint_object.amount -= amount_to_convert;
    //     vault1_object.amount += amount_to_convert;
    //     vault2_object.amount -= converted_value;
    //     wallet_converted_mint_object.amount += converted_value;

    //     msg!("transaction completed, mint 1 dominant: ")
    // } else if wallet_original_mint_object.mint == exchange_booth_object.TokenMint2 {
    //     let converted_value = amount_to_convert * (rate1/rate2);

    //     wallet_original_mint_object.amount -= amount_to_convert;
    //     vault2_object.amount += amount_to_convert;
    //     vault1_object.amount -= converted_value;
    //     wallet_converted_mint_object.amount += converted_value;
        
    //     msg!("transaction completed, mint 2 dominant: {}, {}, {}, {}", wallet_original_mint_object.amount, vault2_object.amount, vault1_object.amount, wallet_converted_mint_object.amount);
    // } else {
    //     msg!("mint object configured improperly");
    // }

    Ok(())
}