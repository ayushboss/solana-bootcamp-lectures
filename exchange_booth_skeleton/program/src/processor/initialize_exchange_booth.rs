use std::mem;

use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    program::invoke_signed,
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
    let mint1 = next_account_info(accounts_iter)?;
    let mint2 = next_account_info(accounts_iter)?;
    let oracle_accnt = next_account_info(accounts_iter)?;
    let vault1 = next_account_info(accounts_iter)?;
    let vault2 = next_account_info(accounts_iter)?;
    let admin_controlled_user_account_mint1 = next_account_info(accounts_iter)?;
    let admin_controlled_user_account_mint2 = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    
    // initialize the vaults as PDAs of this authority program
    // use seeds of this program and specific mint
    
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
            Rent::get()?.minimum_balance(192),
            192,
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
            Rent::get()?.minimum_balance(73),
            73,
            program_id
        ),
        &[admin_signer_account.clone(), vault1.clone(), system_program.clone()],
        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint1.key.as_ref(), &[vault_mint_1_bump]]]
    )?;

    invoke_signed(
        &system_instruction::create_account(
            admin_signer_account.key,
            &vault2.key,
            Rent::get()?.minimum_balance(73),
            73,
            program_id
        ),
        &[admin_signer_account.clone(), vault2.clone(), system_program.clone()],
        &[&[b"vault_from_mint", admin_signer_account.key.as_ref(), cur_exchange_booth_account.key.as_ref(), mint2.key.as_ref(), &[vault_mint_2_bump]]]
    )?;

    let empty_token_account_vault_1 = TokenAccount {
        tag: AccountTag::TokenAccount2,
        amount: 0.0,
        mint: *mint1.key,
        owner: *admin_signer_account.key,
    };

    let empty_token_account_vault_2 = TokenAccount {
        tag: AccountTag::TokenAccount2,
        amount: 0.0,
        mint: *mint2.key,
        owner: *admin_signer_account.key,
    };

    empty_token_account_vault_1.serialize(&mut *vault1.data.borrow_mut())?;
    empty_token_account_vault_2.serialize(&mut *vault2.data.borrow_mut())?;
    
    let mint1_user_token_account = TokenAccount {
        tag: AccountTag::TokenAccount2,
        amount: 2.000,
        mint: *mint1.key,
        owner: *admin_signer_account.key,
    };
    
    let mint2_user_token_account = TokenAccount {
        tag: AccountTag::TokenAccount2,
        amount: 2.000,
        mint: *mint2.key,
        owner: *admin_signer_account.key,
    };

    mint1_user_token_account.serialize(&mut *admin_controlled_user_account_mint1.data.borrow_mut())?;
    mint2_user_token_account.serialize(&mut *admin_controlled_user_account_mint2.data.borrow_mut())?;

    Ok(())
}
