#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("DEHYWjS5ZUM9vPX7ePUnzVq3yqd46Z1m8majoJbyUW9R");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
    }
    
    pub fn refund(ctx: Context<Refund>) -> Result<()>{
        ctx.accounts.refund_and_close_vault()
    }

    pub fn take(ctx: Context<Take> ) -> Result<()>{
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close() 
    }
}

#[derive(Accounts)]
pub struct Initialize {}
