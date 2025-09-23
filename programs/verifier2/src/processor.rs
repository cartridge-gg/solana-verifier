use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use utils::{AccountCast, BidirectionalStack};

use crate::{instruction::VerifierInstruction, state::BidirectionalStackAccount};

/// Program state handler
pub struct Processor;

impl Processor {
    /// Process the initialize instruction
    pub fn process_initialize(accounts: &[AccountInfo]) -> ProgramResult {
        msg!("Processing Initialize instruction");

        // Get the account to initialize
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // Initialize the bidirectional stack
        let mut data = account.try_borrow_mut_data()?;
        let stack_account = BidirectionalStackAccount::cast_mut(*data);

        // Set to default values - front_index to 0, back_index to CAPACITY
        *stack_account = BidirectionalStackAccount::default();
        msg!("Account initialized successfully");

        Ok(())
    }

    /// Process the push task instruction
    pub fn process_push_task(accounts: &[AccountInfo], task_data: Vec<u8>) -> ProgramResult {
        msg!("Processing PushTask instruction");

        // Get the account to push task to
        let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // Push the task to the bidirectional stack
        let mut data = account.try_borrow_mut_data()?;
        let stack_account = BidirectionalStackAccount::cast_mut(*data);

        // Push the task data to the back of the stack
        stack_account.push_back(&task_data).map_err(|e| {
            msg!("Error pushing task: {:?}", e);
            ProgramError::InvalidInstructionData
        })?;
        msg!("Task pushed successfully");

        Ok(())
    }

    /// Process the push data instruction
    pub fn process_push_data(accounts: &[AccountInfo], data_payload: Vec<u8>) -> ProgramResult {
        msg!("Processing PushData instruction");

        // Get the account to push data to
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // Push the data to the bidirectional stack
        let mut data = account.try_borrow_mut_data()?;
        let stack_account = BidirectionalStackAccount::cast_mut(*data);

        // Push the data to the front of the stack
        stack_account.push_front(&data_payload).map_err(|e| {
            msg!("Error pushing data: {:?}", e);
            ProgramError::InvalidInstructionData
        })?;
        msg!("Data pushed successfully");

        Ok(())
    }

    /// Process the execute instruction
    pub fn process_execute(accounts: &[AccountInfo], nonce: u32) -> ProgramResult {
        msg!("Processing Execute instruction, nonce: {}", nonce);

        // Get the account to execute task from
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // Execute the next task in the stack
        let mut data = account.try_borrow_mut_data()?;
        let stack_account = BidirectionalStackAccount::cast_mut(*data);

        // Execute the task
        stack_account.execute();
        msg!("Task executed successfully");

        Ok(())
    }

    pub fn process_set_account_data(
        accounts: &[AccountInfo],
        offset: usize,
        data: Vec<u8>,
    ) -> ProgramResult {
        msg!("Processing SetProof instruction");
        // Get the account to set proof to
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;
        let account_data = &mut account.try_borrow_mut_data()?;

        account_data[offset..offset + data.len()].copy_from_slice(&data);
        msg!("Proof part set successfully");
        Ok(())
    }

    // src: https://github.com/solana-developers/program-examples/blob/main/basics/close-account/native/program/src/instructions/close_user.rs
    pub fn close(accounts: &[AccountInfo]) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let target_account = next_account_info(accounts_iter)?;
        let payer = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        let account_span = 0usize;
        let lamports_required = (Rent::get()?).minimum_balance(account_span);

        let diff = target_account.lamports() - lamports_required;

        // Send the rent back to the payer
        **target_account.lamports.borrow_mut() -= diff;
        **payer.lamports.borrow_mut() += diff;

        // Realloc the account to zero
        target_account.resize(account_span)?;

        // Assign the account to the System Program
        target_account.assign(system_program.key);

        Ok(())
    }
}

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Verifier Rust program entrypoint");

    // Unpack the instruction
    let instruction = VerifierInstruction::try_from_slice(instruction_data)?;

    // Process the instruction
    match instruction {
        VerifierInstruction::SetAccountData(offset, data) => {
            Processor::process_set_account_data(accounts, offset, data)
        }
        VerifierInstruction::PushTask(task_data) => {
            Processor::process_push_task(accounts, task_data)
        }
        VerifierInstruction::PushData(data_payload) => {
            Processor::process_push_data(accounts, data_payload)
        }
        VerifierInstruction::Execute(nonce) => Processor::process_execute(accounts, nonce),

        VerifierInstruction::Close => Processor::close(accounts),
    }
}
