use std::vec;
use wincode::{SchemaWrite, SchemaRead};
use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};

pub const ID: Address = solana_address::address!("22222222222222222222222222222222222222222222");

pub struct MakeInstruction {
    pub maker: Address,
    pub escrow: Address,
    pub mint_a: Address,
    pub mint_b: Address,
    pub maker_ta_a: Address,
    pub maker_ta_b: Address,
    pub vault_ta_a: Address,
    pub rent: Address,
    pub token_program: Address,
    pub system_program: Address,
    pub deposit: u64,
    pub receive: u64,
}

impl From<MakeInstruction> for Instruction {
    fn from(ix: MakeInstruction) -> Instruction {
        let accounts = vec![
            AccountMeta::new(ix.maker, true),
            AccountMeta::new(ix.escrow, false),
            AccountMeta::new_readonly(ix.mint_a, false),
            AccountMeta::new_readonly(ix.mint_b, false),
            AccountMeta::new(ix.maker_ta_a, false),
            AccountMeta::new(ix.maker_ta_b, false),
            AccountMeta::new(ix.vault_ta_a, false),
            AccountMeta::new_readonly(ix.rent, false),
            AccountMeta::new_readonly(ix.token_program, false),
            AccountMeta::new_readonly(ix.system_program, false),
        ];
        let mut data = vec![0];
        data.extend_from_slice(&wincode::serialize(&ix.deposit).unwrap());
        data.extend_from_slice(&wincode::serialize(&ix.receive).unwrap());
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

pub struct TakeInstruction {
    pub taker: Address,
    pub escrow: Address,
    pub maker: Address,
    pub mint_a: Address,
    pub mint_b: Address,
    pub taker_ta_a: Address,
    pub taker_ta_b: Address,
    pub maker_ta_b: Address,
    pub vault_ta_a: Address,
    pub rent: Address,
    pub token_program: Address,
    pub system_program: Address,
}

impl From<TakeInstruction> for Instruction {
    fn from(ix: TakeInstruction) -> Instruction {
        let accounts = vec![
            AccountMeta::new(ix.taker, true),
            AccountMeta::new(ix.escrow, false),
            AccountMeta::new(ix.maker, false),
            AccountMeta::new_readonly(ix.mint_a, false),
            AccountMeta::new_readonly(ix.mint_b, false),
            AccountMeta::new(ix.taker_ta_a, false),
            AccountMeta::new(ix.taker_ta_b, false),
            AccountMeta::new(ix.maker_ta_b, false),
            AccountMeta::new(ix.vault_ta_a, false),
            AccountMeta::new_readonly(ix.rent, false),
            AccountMeta::new_readonly(ix.token_program, false),
            AccountMeta::new_readonly(ix.system_program, false),
        ];
        let data = vec![1];
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

pub struct RefundInstruction {
    pub maker: Address,
    pub escrow: Address,
    pub mint_a: Address,
    pub maker_ta_a: Address,
    pub vault_ta_a: Address,
    pub rent: Address,
    pub token_program: Address,
    pub system_program: Address,
}

impl From<RefundInstruction> for Instruction {
    fn from(ix: RefundInstruction) -> Instruction {
        let accounts = vec![
            AccountMeta::new(ix.maker, true),
            AccountMeta::new(ix.escrow, false),
            AccountMeta::new_readonly(ix.mint_a, false),
            AccountMeta::new(ix.maker_ta_a, false),
            AccountMeta::new(ix.vault_ta_a, false),
            AccountMeta::new_readonly(ix.rent, false),
            AccountMeta::new_readonly(ix.token_program, false),
            AccountMeta::new_readonly(ix.system_program, false),
        ];
        let data = vec![2];
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

pub const ESCROW_ACCOUNT_DISCRIMINATOR: &[u8] = &[1];

#[derive(Clone, Copy, SchemaWrite, SchemaRead)]
#[repr(C)]
pub struct Escrow {
    pub maker: Address,
    pub mint_a: Address,
    pub mint_b: Address,
    pub maker_ta_b: Address,
    pub receive: u64,
    pub bump: u8,
}

pub enum ProgramAccount {
    Escrow(Escrow),
}

pub fn decode_account(data: &[u8]) -> Option<ProgramAccount> {
    if data.starts_with(ESCROW_ACCOUNT_DISCRIMINATOR) {
        let payload = &data[ESCROW_ACCOUNT_DISCRIMINATOR.len()..];
        return wincode::deserialize::<Escrow>(payload).ok().map(ProgramAccount::Escrow);
    }
    None
}

pub const MAKE_EVENT_EVENT_DISCRIMINATOR: &[u8] = &[0];
pub const TAKE_EVENT_EVENT_DISCRIMINATOR: &[u8] = &[1];
pub const REFUND_EVENT_EVENT_DISCRIMINATOR: &[u8] = &[2];

#[derive(SchemaWrite, SchemaRead)]
pub struct MakeEvent {
    pub escrow: Address,
    pub maker: Address,
    pub mint_a: Address,
    pub mint_b: Address,
    pub deposit: u64,
    pub receive: u64,
}

#[derive(SchemaWrite, SchemaRead)]
pub struct TakeEvent {
    pub escrow: Address,
}

#[derive(SchemaWrite, SchemaRead)]
pub struct RefundEvent {
    pub escrow: Address,
}

pub enum ProgramEvent {
    MakeEvent(MakeEvent),
    TakeEvent(TakeEvent),
    RefundEvent(RefundEvent),
}

pub fn decode_event(data: &[u8]) -> Option<ProgramEvent> {
    if data.starts_with(MAKE_EVENT_EVENT_DISCRIMINATOR) {
        let payload = &data[MAKE_EVENT_EVENT_DISCRIMINATOR.len()..];
        return wincode::deserialize::<MakeEvent>(payload).ok().map(ProgramEvent::MakeEvent);
    }
    if data.starts_with(TAKE_EVENT_EVENT_DISCRIMINATOR) {
        let payload = &data[TAKE_EVENT_EVENT_DISCRIMINATOR.len()..];
        return wincode::deserialize::<TakeEvent>(payload).ok().map(ProgramEvent::TakeEvent);
    }
    if data.starts_with(REFUND_EVENT_EVENT_DISCRIMINATOR) {
        let payload = &data[REFUND_EVENT_EVENT_DISCRIMINATOR.len()..];
        return wincode::deserialize::<RefundEvent>(payload).ok().map(ProgramEvent::RefundEvent);
    }
    None
}
