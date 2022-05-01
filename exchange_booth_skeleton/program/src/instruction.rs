use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ExchangeBoothInstruction {
    InititializeExchangeBooth {
        tokenExchange1: f64,
        tokenExchange2: f64,
    },
    Deposit {
        // TODO
    },
    Withdraw {
        // TODO
    },
    Exchange {
        // TODO
    },
    CloseExchangeBooth {
        // TODO
    },
}
