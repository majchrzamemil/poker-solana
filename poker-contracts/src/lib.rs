use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct BuyIn {
    pub stack: u64,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Settle {
    pub final_stack: u64,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum Instruction {
    BuyIn(BuyIn),
    Settle(Settle),
}

impl Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidArgument)?;
        Ok(match tag {
            0 => Self::BuyIn(BuyIn { stack: 0 }),
            1 => Self::Settle(Settle { final_stack: 0 }), //TODO: read final stack later.
            _ => return Err(ProgramError::InvalidArgument),
        })
    }
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Wellcome to POKER!!!!");
    let instruction = Instruction::unpack(instruction_data)?;
    match instruction {
        Instruction::BuyIn(BuyIn { stack }) => msg!("But in called {}", stack),
        Instruction::Settle(Settle { final_stack }) => {
            msg!("Settle called with final stack: {}", final_stack)
        }
    }

    // Iterating accounts is safer than indexing
    //    let accounts_iter = &mut accounts.iter();
    //
    //    // Get the account to say hello to
    //    let account = next_account_info(accounts_iter)?;
    //
    //    // The account must be owned by the program in order to modify its data
    //    if account.owner != program_id {
    //        msg!("Greeted account does not have the correct program id");
    //        return Err(ProgramError::IncorrectProgramId);
    //    }
    //
    //    // Increment and store the number of times the account has been greeted
    //    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    //    greeting_account.counter += 1;
    //    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    //
    //    msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}
