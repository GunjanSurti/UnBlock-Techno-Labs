use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{approve, transfer, Approve, Mint, Token, TokenAccount, Transfer},
};

pub fn to_approve(ctx: Context<ToApprove>, amt: u64) -> Result<()> {
    approve(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Approve {
                to: ctx.accounts.to.to_account_info(),
                delegate: ctx.accounts.delegate.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amt,
    )?;
    msg!("---------------------------Aproved----------------------------------");
    Ok(())
}
#[derive(Accounts)]
pub struct ToApprove<'info> {
    #[account(mut)]
    pub delegate: SystemAccount<'info>, // this is public key of one who can transfer on belalf

    #[account(mut)]
    pub to: Account<'info, TokenAccount>, // this is ATA of which we want to delegate

    pub authority: SystemAccount<'info>, // authority of "to"

    pub token_program: Program<'info, Token>, // TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
}

pub fn transfer_approved_tokens(ctx: Context<TransferApprovedTokens>, amount: u64) -> Result<()> {
    msg!("Transferring tokens...");
    msg!(
        "Mint: {}",
        &ctx.accounts.mint_account.to_account_info().key()
    );
    msg!(
        "From Token Address: {}",
        &ctx.accounts.sender_token_account.key()
    );
    msg!(
        "To Token Address: {}",
        &ctx.accounts.recipient_token_account.key()
    );

    // Invoke the transfer instruction on the token program
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        amount * 10u64.pow(ctx.accounts.mint_account.decimals as u32), // Transfer amount, adjust for decimals
    )?;

    msg!("Tokens transferred successfully.");

    Ok(())
}

#[derive(Accounts)]
pub struct TransferApprovedTokens<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    pub recipient: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    #[account(mut,)]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(mut,)]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>, // TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
    pub associated_token_program: Program<'info, AssociatedToken>, // ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
    pub system_program: Program<'info, System>, // 11111111111111111111111111111111
}
