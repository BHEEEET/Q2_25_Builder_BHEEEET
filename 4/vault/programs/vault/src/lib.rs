#![allow(unexpected_cfgs)]
use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("G1bqdmqpYqopodfmhe89UP3NYquTSGFwWE6Ji1VWtZJv");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()>{
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()>{
        ctx.accounts.withdraw(amount)
    }

    pub fn close(ctx: Context<Close>) -> Result<()>{
        ctx.accounts.close()
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // Not need to init, because we are not sending SOL to an account
    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,

    )]
    pub vault: SystemAccount<'info>,

    // init  makes it also mutable
    #[account(
        init,
        payer = signer,
        seeds = [b"state", signer.key().as_ref()],
        bump,
        space = 8 + VaultState::INIT_SPACE,
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,

    )]
    pub vault: SystemAccount<'info>,

    // init  makes it also mutable
    #[account(
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,

    )]
    // holding lamports
    pub vault: SystemAccount<'info>,

    // init  makes it also mutable
    #[account(
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    // holding bumps
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    // holding lamports
    pub vault: SystemAccount<'info>,

    // mut because we are sending sol
    #[account(
        mut,
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
        // close macro, to send lamports to signer
        close = signer
    )]
    // holding bumps
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
}

impl<'info> Initialize<'info>{
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()>{
        self.vault_state.state_bump = bumps.vault_state;
        self.vault_state.vault_bump = bumps.vault;

        Ok(())
    }
}


impl<'info> Deposit<'info>{
    pub fn deposit(&mut self, amount: u64) -> Result<()>{
        let cpi_program = self.system_program.to_account_info();

        let cpi_account = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

impl<'info> Withdraw<'info>{
    pub fn withdraw(&mut self, amount: u64) -> Result<()>{
        let cpi_program = self.system_program.to_account_info();

        let cpi_account = Transfer {
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info()
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]
            ];

        let signer_seeds = &[&seeds[..]];

        // if you are signing with pda you need to do cpi_contect::new_with_signer
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account, signer_seeds);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

impl<'info> Close<'info>{
    pub fn close(&mut self) -> Result<()>{
        let cpi_program = self.system_program.to_account_info();

        let cpi_account = Transfer {
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info()
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]
            ];

        let signer_seeds = &[&seeds[..]];

        let amount = self.vault.lamports();

        // if you are signing with pda you need to do cpi_contect::new_with_signer
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account, signer_seeds);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[account]
pub struct VaultState{
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState{
    // u8 = 1, 
    const INIT_SPACE: usize = 1 + 1;
}
