use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ExchangeBoothInstruction {
    InititializeExchangeBooth {
        tokenExchange1: f64,
        tokenExchange2: f64,
     },
    Deposit {
        transfer_amount: f64,
    },
    Withdraw {
        withdrawal_amount: f64,
    },
    Exchange {
        // TODO
        amount_to_convert: u64,
        amount_to_convert_scale: u64,
    },
    CloseExchangeBooth {
        // TODO
    },
}
