use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ExchangeBooth {
    pub TokenMint1: Pubkey,
    pub TokenMint2: Pubkey,
    pub OracleSeed: Pubkey,
    pub Admin: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct OracleModel {
    pub TokenMint1: Pubkey,
    pub TokenMint2: Pubkey,
    pub Mint1ExchangeRate: f64,
    pub Mint2ExchangeRate: f64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum AccountTag {
    Uninitialized,
    Mint,
    TokenAccount2,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TokenAccount {
    pub tag: AccountTag,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: f64,
}
//Questions:
// should we make the mint a pubkey object or just a u8 for a seed