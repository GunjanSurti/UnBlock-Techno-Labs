use anchor_lang::prelude::*;
use anchor_spl::token::{approve, set_authority, SetAuthority, Token};
use spl_token::instruction::AuthorityType;

pub fn set_edition_authority_mint_freez(ctx: Context<Authority>) -> Result<()> {
    let cpi_auth = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        SetAuthority {
            account_or_mint: ctx.accounts.current_account.to_account_info(),
            current_authority: ctx.accounts.current_auth.to_account_info(),
        },
    );
    let cpi_auth2 = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        SetAuthority {
            account_or_mint: ctx.accounts.current_account.to_account_info(),
            current_authority: ctx.accounts.current_auth.to_account_info(),
        },
    );
    set_authority(
        cpi_auth,
        AuthorityType::MintTokens,
        Some(ctx.accounts.edition_account.key()),
    );
    set_authority(
        cpi_auth2,
        AuthorityType::FreezeAccount,
        Some(ctx.accounts.edition_account.key()),
    );
    msg!("---------------------------Changed mint and freez authority----------------------------------");

    Ok(())
}

#[derive(Accounts)]
pub struct Authority<'info> {
    #[account(mut)]
    pub edition_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub current_auth: UncheckedAccount<'info>,

    #[account(mut)]
    pub current_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}
