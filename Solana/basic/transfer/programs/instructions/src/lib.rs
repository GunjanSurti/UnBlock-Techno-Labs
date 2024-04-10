#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod instructions;
use solana_program::pubkey::Pubkey;
// use anchor_lang::accounts::interface_account::InterfaceAccount;
use anchor_spl::{
    associated_token::get_associated_token_address_with_program_id,
    token::{approve_checked, transfer_checked, ApproveChecked, Mint, Token, TransferChecked},
    // token_interface::{Mint as MMint, TokenAccount, TokenInterface},
};
use instructions::*;
declare_id!("GfTUynWspq4nrrSfvSjJcXfNZuvmygCtgeNQbDCRVn8y");

#[program]
pub mod transfer_tokens {
    use super::*;

    pub fn create_token(
        ctx: Context<CreateToken>,
        token_title: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        create::create_token(ctx, token_title, token_symbol, token_uri)
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        mint::mint_token(ctx, amount)
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        transfer::transfer_tokens(ctx, amount)
    }

    pub fn get_ata_with_id(ctx: Context<ATA>) -> Result<()> {
        let pg_td = ctx.program_id.key();
        msg!("--------------{}", pg_td);
        let new_ata = get_associated_token_address_with_program_id(
            &ctx.accounts.wallet_address.key(),
            &ctx.accounts.mint.key(),
            &ctx.accounts.token_program_id.key(),
        );
        msg!("{}", new_ata);
        Ok(())
    }

    pub fn just_approve(ctx: Context<ToApprove>, amount: u64) -> Result<()> {
        aprove::to_approve(ctx, amount)
    }
    pub fn transfer_approve(ctx: Context<TransferApprovedTokens>, amount: u64) -> Result<()> {
        aprove::transfer_approved_tokens(ctx, amount)
    }

    // pub fn to_transfer_checked_2(
    //     ctx: Context<CheckedTransfer>,
    //     amount: u64,
    //     // decimals: u8,
    // ) -> Result<()> {
    //     msg!("Transfered via TransferedChecked----------------------------------");
    //     to_transfer_checked(ctx, amount)
    // }

    // pub fn to_approve<'info>(
    //     ctx: CpiContext<'info, 'info, 'info, 'info, ApproveChecked<'info>>,
    //     amount: u64,
    // ) -> Result<()> {
    //     approve_checked(ctx, amount, decimals);
    //     msg!("Aproved---------------------------------------------");
    //     Ok(())
    // }

    // pub fn to_transfer_checked<'info>(
    //     ctx: CpiContext<'info, 'info, 'info, 'info, TransferChecked<'info>>,
    //     amount: u64,
    //     decimals: u8,
    // ) -> Result<()> {
    //     transfer_checked(ctx, amount, decimals);
    //     msg!("Transfered------------------------------------------");

    //     Ok(())
    // }

}

#[derive(Accounts)]
pub struct ATA<'info> {
    // #[account(mut)]
    // pub payer: Signer<'info>,
    #[account(mut)]
    pub wallet_address: SystemAccount<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub token_program_id: Program<'info, Token>, // TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
}
