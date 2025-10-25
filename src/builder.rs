use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
    message::Message,
    hash::Hash,
};
use spl_token::instruction as token_instruction;
use spl_associated_token_account::instruction as ata_instruction;
use rust_decimal::prelude::ToPrimitive;

use crate::{
    transfer::TransferRequest,
    error::{Result, SolanaPayError},
};

/// Utility functions for creating Solana transactions from Solana Pay requests
pub struct TransactionBuilder;

impl TransactionBuilder {
    /// Create a SOL transfer transaction from a TransferRequest
    pub fn create_sol_transfer(
        request: &TransferRequest,
        payer: &Pubkey,
        _recent_blockhash: Hash,
    ) -> Result<Transaction> {
        let amount = request.amount
            .ok_or_else(|| SolanaPayError::MissingField("amount".to_string()))?;

        // Convert to lamports (1 SOL = 1_000_000_000 lamports)
        let lamports = Self::decimal_to_lamports(amount)?;

        let mut instructions = Vec::new();

        // Create transfer instruction
        let transfer_instruction = system_instruction::transfer(
            payer,
            &request.recipient,
            lamports,
        );
        instructions.push(transfer_instruction);

        // Add memo instruction if provided
        if let Some(memo) = &request.memo {
            let memo_instruction = Self::create_memo_instruction(memo, &[payer])?;
            instructions.push(memo_instruction);
        }

        let message = Message::new(&instructions, Some(payer));
        Ok(Transaction::new_unsigned(message))
    }

    /// Create an SPL token transfer transaction from a TransferRequest
    pub fn create_token_transfer(
        request: &TransferRequest,
        payer: &Pubkey,
        token_decimals: u8,
        recent_blockhash: Hash,
    ) -> Result<Transaction> {
        let mint = request.spl_token
            .ok_or_else(|| SolanaPayError::MissingField("spl_token".to_string()))?;
        
        let amount = request.amount
            .ok_or_else(|| SolanaPayError::MissingField("amount".to_string()))?;

        // Convert to token units based on decimals
        let token_amount = Self::decimal_to_token_amount(amount, token_decimals)?;

        let mut instructions = Vec::new();

        // Get associated token accounts
        let sender_ata = spl_associated_token_account::get_associated_token_address(payer, &mint);
        let recipient_ata = spl_associated_token_account::get_associated_token_address(&request.recipient, &mint);

        // Create recipient ATA if it doesn't exist
        let create_ata_instruction = ata_instruction::create_associated_token_account(
            payer,
            &request.recipient,
            &mint,
            &spl_token::id(),
        );
        instructions.push(create_ata_instruction);

        // Create transfer instruction
        let transfer_instruction = token_instruction::transfer(
            &spl_token::id(),
            &sender_ata,
            &recipient_ata,
            payer,
            &[],
            token_amount,
        )?;
        instructions.push(transfer_instruction);

        // Add memo instruction if provided
        if let Some(memo) = &request.memo {
            let memo_instruction = Self::create_memo_instruction(memo, &[payer])?;
            instructions.push(memo_instruction);
        }

        let message = Message::new(&instructions, Some(payer));
        Ok(Transaction::new_unsigned(message))
    }

    /// Convert decimal amount to lamports
    fn decimal_to_lamports(amount: Decimal) -> Result<u64> {
        let lamports_per_sol = Decimal::new(1_000_000_000, 0);
        let lamports = amount * lamports_per_sol;
        
        lamports.to_u64()
            .ok_or_else(|| SolanaPayError::InvalidAmount("Amount too large".to_string()))
    }

    /// Convert decimal amount to token units based on decimals
    fn decimal_to_token_amount(amount: Decimal, decimals: u8) -> Result<u64> {
        let multiplier = Decimal::new(10_u64.pow(decimals as u32) as i64, 0);
        let token_amount = amount * multiplier;
        
        token_amount.to_u64()
            .ok_or_else(|| SolanaPayError::InvalidAmount("Amount too large".to_string()))
    }

    /// Create a memo instruction
    fn create_memo_instruction(memo: &str, signers: &[&Pubkey]) -> Result<Instruction> {
        Ok(Instruction {
            program_id: spl_memo::id(),
            accounts: signers.iter().map(|&signer| AccountMeta::new_readonly(*signer, true)).collect(),
            data: memo.as_bytes().to_vec(),
        })
    }

    /// Validate that a transaction matches the TransferRequest
    pub fn validate_transaction(
        transaction: &Transaction,
        request: &TransferRequest,
        expected_payer: &Pubkey,
    ) -> Result<()> {
        // Basic validation - this is a simplified version
        // In practice, you'd want more thorough validation
        
        if transaction.message.account_keys.is_empty() {
            return Err(SolanaPayError::InvalidUrl("Transaction has no accounts".to_string()));
        }

        // Check that the payer is correct
        if transaction.message.account_keys[0] != *expected_payer {
            return Err(SolanaPayError::InvalidUrl("Incorrect payer".to_string()));
        }

        // Check that recipient is in the account keys
        if !transaction.message.account_keys.contains(&request.recipient) {
            return Err(SolanaPayError::InvalidUrl("Recipient not found in transaction".to_string()));
        }

        Ok(())
    }
}