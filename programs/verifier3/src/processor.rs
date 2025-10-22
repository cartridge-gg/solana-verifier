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

    /// Copy data from another account to our account
    pub fn process_copy_from_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        msg!("Processing CopyFromAccount instruction");

        let accounts_iter = &mut accounts.iter();
        let source_account = next_account_info(accounts_iter)?;
        let dest_account = next_account_info(accounts_iter)?;

        // Verify destination account is owned by this program
        if dest_account.owner != program_id {
            msg!("Error: Destination account not owned by this program");
            return Err(ProgramError::IncorrectProgramId);
        }

        // Verify destination is writable
        if !dest_account.is_writable {
            msg!("Error: Destination account not writable");
            return Err(ProgramError::InvalidAccountData);
        }

        // Borrow data (source is readonly, dest is mut)
        let src_data = source_account.try_borrow_data()?;
        let mut dst_data = dest_account.try_borrow_mut_data()?;

        // Verify size
        if dst_data.len() < src_data.len() {
            msg!(
                "Error: Destination too small ({} < {})",
                dst_data.len(),
                src_data.len()
            );
            return Err(ProgramError::AccountDataTooSmall);
        }

        // Copy data
        dst_data[..src_data.len()].copy_from_slice(&src_data);
        msg!(
            "Successfully copied {} bytes from {} to {}",
            src_data.len(),
            source_account.key,
            dest_account.key
        );

        Ok(())
    }

    /// Transfer ownership of an account to a new program
    /// Following the pattern: resize(0) -> assign -> resize(original_size)
    pub fn process_transfer_ownership(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        msg!("Processing TransferOwnership instruction");

        let accounts_iter = &mut accounts.iter();
        let target_account = next_account_info(accounts_iter)?;
        let new_owner_account = next_account_info(accounts_iter)?;

        // Verify current account is owned by this program
        if target_account.owner != program_id {
            msg!("Error: Account not owned by this program");
            return Err(ProgramError::IncorrectProgramId);
        }

        // Step 1: Assign to new owner
        msg!("Step 1: Assigning to new owner: {}", new_owner_account.key);
        target_account.assign(new_owner_account.key);

        // Step 2: Zero out all data manually
        msg!("Step 2: Zeroing out all data");
        let mut data = target_account.try_borrow_mut_data()?;
        data.fill(0);

        msg!(
            "Successfully transferred ownership of {} to {}",
            target_account.key,
            new_owner_account.key
        );

        Ok(())
    }

    /// Clear account data by resizing to 0 and back to original size
    /// This completely clears the account while preserving ownership and size
    pub fn process_clear_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        msg!("Processing ClearAccount instruction");

        let accounts_iter = &mut accounts.iter();
        let target_account = next_account_info(accounts_iter)?;

        // Verify current account is owned by this program
        if target_account.owner != program_id {
            msg!("Error: Account not owned by this program");
            return Err(ProgramError::IncorrectProgramId);
        }

        // Save original size
        let original_size = target_account.data_len();
        msg!("Original account size: {}", original_size);

        // Step 1: Resize to zero (clears all data)
        msg!("Step 1: Resizing to 0 bytes (clearing data)");
        target_account.resize(0)?;

        msg!("Step 2: Resizing back to {} bytes in chunks", original_size);
        const MAX_CHUNK_SIZE: usize = 10_240; // MAX_PERMITTED_DATA_INCREASE
        let mut current_size = 0;
        
        while current_size < original_size {
            let chunk_size = std::cmp::min(MAX_CHUNK_SIZE, original_size - current_size);
            let new_size = current_size + chunk_size;
            
            msg!("  Resizing to {} bytes (chunk: {})", new_size, chunk_size);
            target_account.resize(new_size)?;
            
            current_size = new_size;
        }

        msg!(
            "Successfully cleared account {} ({} bytes)",
            target_account.key,
            original_size
        );

        Ok(())
    }
}

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
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
        VerifierInstruction::CopyFromAccount => {
            Processor::process_copy_from_account(program_id, accounts)
        }
        VerifierInstruction::TransferOwnership => {
            Processor::process_transfer_ownership(program_id, accounts)
        }
        VerifierInstruction::ClearAccount => {
            Processor::process_clear_account(program_id, accounts)
        }
        VerifierInstruction::Close => Processor::close(accounts),
    }
}
