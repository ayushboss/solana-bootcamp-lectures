use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, program_error::ProgramError,
    system_instruction,
    pubkey::Pubkey,
    program::{invoke_signed, invoke},
    sysvar::{rent::Rent, Sysvar},
};

use crate::error::EchoError;
use crate::instruction::EchoInstruction;
use crate::state::AuthorizedBufferHeader;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EchoInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;


        match instruction {
            EchoInstruction::Echo { data } => {
                let accounts_iter = &mut _accounts.iter();
                let current_account = next_account_info(accounts_iter)?;

                let mut ctr = 0;
                for c in current_account.data.borrow_mut().iter_mut() {
                    *c = data[ctr];
                    ctr+=1;
                }
                
                Ok(())
            }
            EchoInstruction::InitializeAuthorizedEcho {
                buffer_seed,
                buffer_size,
            } => {
                msg!("Instruction: InitializeAuthorizedEcho");
                msg!("{}", buffer_seed);
                msg!("{}", buffer_size);
                let accounts_iter = &mut _accounts.iter();
                let authorized_buffer = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;
                let system_program = next_account_info(accounts_iter)?;

                let (authorithed_buffer_key, bump_seed) = Pubkey::find_program_address(
                    &[
                        b"authority",
                        authority.key.as_ref(),
                        &buffer_seed.to_le_bytes()
                    ],
                    _program_id,
                );

                assert_eq!(authorithed_buffer_key, *authorized_buffer.key);
                invoke_signed(
                    &system_instruction::create_account(
                        authority.key,
                        authorized_buffer.key,
                        Rent::get()?.minimum_balance(buffer_size),
                        buffer_size.try_into().unwrap(),
                        _program_id,
                    ),
                    &[authority.clone(), authorized_buffer.clone(), system_program.clone()],
                    &[&[b"authority", authority.key.as_ref(), &buffer_seed.to_le_bytes(), &[bump_seed]]],
                )?;

                let thing = AuthorizedBufferHeader {
                    bp_seed: bump_seed,
                    buf_seed: buffer_seed,
                };

                thing.serialize(&mut *authorized_buffer.data.borrow_mut())?;
                Ok(())
            }
            EchoInstruction::AuthorizedEcho { data } => {
                msg!("Instruction: AuthorizedEcho");
                let accounts_iter = &mut _accounts.iter();
                let authorized_buffer = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;
                let (authorized_buffer_key, bump_seed) = Pubkey::find_program_address(
                    &[
                        b"authority",
                        authority.key.as_ref(),
                        &authorized_buffer.data.borrow()[1..=8],
                    ],
                    _program_id
                );
                
                assert_eq!(authorized_buffer_key, *authorized_buffer.key);

                
                for (ch, ptr) in data.iter().zip(authorized_buffer.data.borrow_mut()[9..].iter_mut()) {
                    *ptr = *ch;
                }
                Ok(())
            }
            EchoInstruction::InitializeVendingMachineEcho {
                price: _,
                buffer_size: _,
            } => {
                msg!("Instruction: InitializeVendingMachineEcho");
                Err(EchoError::NotImplemented.into())
            }
            EchoInstruction::VendingMachineEcho { data: _ } => {
                msg!("Instruction: VendingMachineEcho");
                Err(EchoError::NotImplemented.into())
            }
        }
    }
}
