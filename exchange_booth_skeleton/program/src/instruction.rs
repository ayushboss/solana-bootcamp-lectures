use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ExchangeBoothInstruction {
    InititializeExchangeBooth {
        tokenExchange1: f64,
        tokenExchange2: f64,
    },
    Deposit {
        // TODO
        deposit_amount: f64,
    },
    Withdraw {
        // TODO
        withdrawal_amount: f64,
    },
    Exchange {
        // TODO
        num_tokens_to_deposit: f64,
    },
    CloseExchangeBooth {
        // TODO
    },
}
