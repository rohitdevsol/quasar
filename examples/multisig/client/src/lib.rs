use std::vec;
use std::vec::Vec;
use quasar_lang::client::{DynBytes, DynVec};
use wincode::{SchemaWrite, SchemaRead};
use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};

pub const ID: Address = solana_address::address!("44444444444444444444444444444444444444444444");

pub struct CreateInstruction {
    pub creator: Address,
    pub config: Address,
    pub rent: Address,
    pub system_program: Address,
    pub threshold: u8,
    pub remaining_accounts: Vec<AccountMeta>,
}

impl From<CreateInstruction> for Instruction {
    fn from(ix: CreateInstruction) -> Instruction {
        let mut accounts = vec![
            AccountMeta::new(ix.creator, true),
            AccountMeta::new(ix.config, false),
            AccountMeta::new_readonly(ix.rent, false),
            AccountMeta::new_readonly(ix.system_program, false),
        ];
        accounts.extend(ix.remaining_accounts);
        let mut data = vec![0];
        data.extend_from_slice(&wincode::serialize(&ix.threshold).unwrap());
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

pub struct DepositInstruction {
    pub depositor: Address,
    pub config: Address,
    pub vault: Address,
    pub system_program: Address,
    pub amount: u64,
}

impl From<DepositInstruction> for Instruction {
    fn from(ix: DepositInstruction) -> Instruction {
        let accounts = vec![
            AccountMeta::new(ix.depositor, true),
            AccountMeta::new_readonly(ix.config, false),
            AccountMeta::new(ix.vault, false),
            AccountMeta::new_readonly(ix.system_program, false),
        ];
        let mut data = vec![1];
        data.extend_from_slice(&wincode::serialize(&ix.amount).unwrap());
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

pub struct SetLabelInstruction {
    pub creator: Address,
    pub config: Address,
    pub system_program: Address,
    pub label: DynBytes,
}

impl From<SetLabelInstruction> for Instruction {
    fn from(ix: SetLabelInstruction) -> Instruction {
        let accounts = vec![
            AccountMeta::new(ix.creator, true),
            AccountMeta::new(ix.config, false),
            AccountMeta::new_readonly(ix.system_program, false),
        ];
        let mut data = vec![2];
        data.extend_from_slice(&wincode::serialize(&ix.label).unwrap());
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

pub struct ExecuteTransferInstruction {
    pub config: Address,
    pub creator: Address,
    pub vault: Address,
    pub recipient: Address,
    pub system_program: Address,
    pub amount: u64,
    pub remaining_accounts: Vec<AccountMeta>,
}

impl From<ExecuteTransferInstruction> for Instruction {
    fn from(ix: ExecuteTransferInstruction) -> Instruction {
        let mut accounts = vec![
            AccountMeta::new_readonly(ix.config, false),
            AccountMeta::new_readonly(ix.creator, false),
            AccountMeta::new(ix.vault, false),
            AccountMeta::new(ix.recipient, false),
            AccountMeta::new_readonly(ix.system_program, false),
        ];
        accounts.extend(ix.remaining_accounts);
        let mut data = vec![3];
        data.extend_from_slice(&wincode::serialize(&ix.amount).unwrap());
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

pub const MULTISIG_CONFIG_ACCOUNT_DISCRIMINATOR: &[u8] = &[1];

#[derive(Clone, SchemaWrite, SchemaRead)]
pub struct MultisigConfig {
    pub creator: Address,
    pub threshold: u8,
    pub bump: u8,
    pub label: DynBytes,
    pub signers: DynVec<Address>,
}

pub enum ProgramAccount {
    MultisigConfig(MultisigConfig),
}

pub fn decode_account(data: &[u8]) -> Option<ProgramAccount> {
    if data.starts_with(MULTISIG_CONFIG_ACCOUNT_DISCRIMINATOR) {
        let payload = &data[MULTISIG_CONFIG_ACCOUNT_DISCRIMINATOR.len()..];
        return wincode::deserialize::<MultisigConfig>(payload).ok().map(ProgramAccount::MultisigConfig);
    }
    None
}
