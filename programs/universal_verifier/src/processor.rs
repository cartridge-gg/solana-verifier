use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use utils::{AccountCast, BidirectionalStack, UniversalStackAccount, VerifierMode};

use crate::instruction::{UniversalVerifierInstruction, VerifierInstruction};
/// Program state handler
pub struct Processor;

impl Processor {
    /// Process the switch mode instruction
    pub fn process_switch_mode(accounts: &[AccountInfo], new_mode: VerifierMode) -> ProgramResult {
        msg!("Processing SwitchMode instruction to {:?}", new_mode);

        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        let mut data = account.try_borrow_mut_data()?;
        let stack_account = UniversalStackAccount::cast_mut(*data);

        // Switch the mode
        stack_account.switch_mode(new_mode);
        msg!("Mode switched to {:?}", new_mode);

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
        let stack_account = UniversalStackAccount::cast_mut(*data);

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
        let stack_account = UniversalStackAccount::cast_mut(*data);

        // Push the data to the front of the stack
        stack_account.push_front(&data_payload).map_err(|e| {
            msg!("Error pushing data: {:?}", e);
            ProgramError::InvalidInstructionData
        })?;
        msg!("Data pushed successfully");

        Ok(())
    }

    /// Process the execute instruction - delegates to appropriate verifier via CPI
    pub fn process_execute(accounts: &[AccountInfo], nonce: u32) -> ProgramResult {
        msg!("Processing Execute instruction, nonce: {}", nonce);

        // Get the stack account
        let accounts_iter = &mut accounts.iter();
        let stack_account = next_account_info(accounts_iter)?;

        // Read the current mode to determine which verifier to call
        let mode = {
            let data = stack_account.try_borrow_data()?;
            let stack = UniversalStackAccount::cast(*data);
            stack.mode()
        };

        msg!("Current verifier mode: {:?}", mode);

        // Think how to get the verifier program id
        let verifier_program_id = Pubkey::new_from_array([0; 32]);

        msg!("Delegating to verifier program: {}", verifier_program_id);

        let execute_ix = Instruction::new_with_borsh(
            verifier_program_id,
            &VerifierInstruction::Execute(nonce),
            vec![AccountMeta::new(*stack_account.key, false)],
        );

        // Invoke the verifier program via CPI
        invoke(&execute_ix, &[stack_account.clone()])?;

        msg!("Task executed successfully via CPI");

        Ok(())
    }

    pub fn process_set_account_data(
        accounts: &[AccountInfo],
        offset: usize,
        data: Vec<u8>,
    ) -> ProgramResult {
        msg!("Processing SetAccountData instruction");

        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;
        let account_data = &mut account.try_borrow_mut_data()?;

        account_data[offset..offset + data.len()].copy_from_slice(&data);
        msg!("Account data set successfully");

        Ok(())
    }

    /// Process the execute with specific program ID instruction
    pub fn process_execute_with_program_id(
        accounts: &[AccountInfo],
        nonce: u32,
        verifier_program_id: Pubkey,
    ) -> ProgramResult {
        msg!(
            "Processing ExecuteWithProgramId instruction, nonce: {}, program_id: {}",
            nonce,
            verifier_program_id
        );

        // Get the stack account and verifier program account
        let accounts_iter = &mut accounts.iter();
        let stack_account = next_account_info(accounts_iter)?;
        let verifier_program_account = next_account_info(accounts_iter)?;

        let execute_ix = Instruction::new_with_borsh(
            verifier_program_id,
            &VerifierInstruction::Execute(nonce),
            vec![
                AccountMeta::new(*stack_account.key, false),
                AccountMeta::new(*verifier_program_account.key, false),
            ],
        );

        // Invoke the verifier program via CPI
        invoke(
            &execute_ix,
            &[stack_account.clone(), verifier_program_account.clone()],
        )?;

        msg!(
            "Task executed successfully via CPI with program ID: {}",
            verifier_program_id
        );

        Ok(())
    }

    /// Process test CPI instruction
    pub fn process_test_cpi(
        accounts: &[AccountInfo],
        verifier_program_id: Pubkey,
    ) -> ProgramResult {
        msg!(
            "Processing TestCPI instruction with verifier program: {}",
            verifier_program_id
        );

        let accounts_iter = &mut accounts.iter();
        let stack_account = next_account_info(accounts_iter)?;
        let verifier_program = next_account_info(accounts_iter)?;

        // Read front_index before CPI
        let front_index_before = {
            let data = stack_account.try_borrow_data()?;
            let stack = UniversalStackAccount::cast(*data);
            stack.front_index
        };
        msg!("Before CPI: front_index = {}", front_index_before);

        // Create TestExecute instruction for verifier1
        // Pass stack_account as signer since it signed the original transaction
        let test_execute_ix = Instruction::new_with_borsh(
            verifier_program_id,
            &VerifierInstruction::TestExecute,
            vec![AccountMeta::new(
                *stack_account.key,
                stack_account.is_signer,
            )],
        );

        // Invoke verifier1 via CPI
        msg!("Calling verifier1::TestExecute via CPI...");
        invoke(
            &test_execute_ix,
            &[stack_account.clone(), verifier_program.clone()],
        )?;

        // Read front_index after CPI
        let front_index_after = {
            let data = stack_account.try_borrow_data()?;
            let stack = UniversalStackAccount::cast(*data);
            stack.front_index
        };
        msg!("After CPI: front_index = {}", front_index_after);

        if front_index_after == front_index_before + 1 {
            msg!("✓ CPI TEST PASSED: verifier1 successfully modified account owned by universal_verifier!");
        } else {
            msg!("✗ CPI TEST FAILED: front_index did not increment as expected");
            return Err(ProgramError::Custom(999));
        }

        Ok(())
    }

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
    msg!("Universal Verifier program entrypoint");

    // Unpack the instruction
    let instruction = UniversalVerifierInstruction::try_from_slice(instruction_data)?;

    // Process the instruction
    match instruction {
        UniversalVerifierInstruction::SwitchMode(mode) => {
            Processor::process_switch_mode(accounts, mode)
        }
        UniversalVerifierInstruction::SetAccountData(offset, data) => {
            Processor::process_set_account_data(accounts, offset, data)
        }
        UniversalVerifierInstruction::PushTask(task_data) => {
            Processor::process_push_task(accounts, task_data)
        }
        UniversalVerifierInstruction::PushData(data_payload) => {
            Processor::process_push_data(accounts, data_payload)
        }
        UniversalVerifierInstruction::Execute(nonce) => Processor::process_execute(accounts, nonce),
        UniversalVerifierInstruction::ExecuteWithProgramId(nonce, program_id_bytes) => {
            let program_id = Pubkey::new_from_array(program_id_bytes);
            Processor::process_execute_with_program_id(accounts, nonce, program_id)
        }
        UniversalVerifierInstruction::TestCPI(program_id_bytes) => {
            let program_id = Pubkey::new_from_array(program_id_bytes);
            Processor::process_test_cpi(accounts, program_id)
        }
        UniversalVerifierInstruction::Close => Processor::close(accounts),
    }
}
