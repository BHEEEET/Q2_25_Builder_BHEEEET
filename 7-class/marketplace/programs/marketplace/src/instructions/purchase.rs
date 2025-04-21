use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;
use anchor_spl::token_interface::{Mint, TokenInterface};
use anchor_spl::token::{close_account, transfer, transfer_checked, CloseAccount, Transfer, TransferChecked};

use crate::state::Marketplace;
use crate::state::Listing;

#[derive(Accounts)]
pub struct Purchase<'info>{
    #[account(mut)]
   pub taker: Signer<'info>,
   pub maker: Signer<'info>,
   #[account(
    seeds = [b"marketplace", marketplace.name.as_bytes()],
    bump,
   )]
    pub marketplace: Account<'info, Marketplace>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(

    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    // treasury is a systemaccount because we are accepting LAMPORTS (SOL)
    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
        close = maker
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        mut,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump = marketplace.rewards_bump,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>
}

impl<'info> Purchase<'info>{
    pub fn send_sol(&self) -> Result<()>{

        // calculate marketplace fee
        let marketplace_fee = (self.marketplace.fee as u64)
            .checked_mul(self.listing.price) // checks overflow
            .unwrap()
            .checked_div(10000_u64)
            .unwrap();

        let cpi_progam = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
            authority: self.taker.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_progam, cpi_accounts);

        let amount = self.listing.price.checked_sub(marketplace_fee).unwrap();

        // send sol to maker
        transfer(cpi_ctx, amount)?;

        // sending marketplace fee to treasury
        let cpi_fee_program = self.system_program.to_account_info();

        let cpi_fee_accounts = Transfer{
            from: self.taker.to_account_info(),
            to: self.treasury.to_account_info(),
            authority: self.taker.to_account_info()
        };

        let cpi_fee_ctx = CpiContext::new(cpi_fee_program, cpi_fee_accounts);


        transfer(cpi_fee_ctx, marketplace_fee)
    }

    pub fn send_nft(&mut self) -> Result<()>{
        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked{
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)
    }

    pub fn close_mint_vault(&mut self) -> Result<()>{
        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();

        // Close account struct
        let cpi_accounts = CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // close account function
        close_account(cpi_ctx)
    }
}