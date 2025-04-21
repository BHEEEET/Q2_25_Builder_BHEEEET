#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
mod instructions;
mod state;
use crate::instructions::*;

declare_id!("445XsBpNeYBDAci13DSbisZpXjWAgeFpM7jTfpMeEjjR");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)
    }

    pub fn create_listing(ctx: Context<List>, price: u64) -> Result<()>{
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()>{
        ctx.accounts.send_sol()?;
        ctx.accounts.send_nft()?;
        ctx.accounts.close_mint_vault()
    }
    
    pub fn delist(ctx: Context<Delist>) -> Result<()>{
        ctx.accounts.withdraw_nft()?;
        ctx.accounts.close_listing()
    }
}