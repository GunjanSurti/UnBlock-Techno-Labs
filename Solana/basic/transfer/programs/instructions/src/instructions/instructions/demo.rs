use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{Mint, Token, TokenAccount},
};
// use mpl_token_metadata::accounts::MasterEdition;
use mpl_token_metadata::instructions::{CreateMasterEditionV3CpiBuilder};
// 1. every account is specified by a reference to their AccountInfo
pub fn create_next_thing(ctx: Context<GHH>) -> Result<()> {
    let master_builder = CreateMasterEditionV3CpiBuilder::new(
        &ctx.accounts.token_metadata_program.to_account_info(),
    ).edition(&ctx.accounts.edition_account.to_account_info())
    .mint(&ctx.accounts.mint_account.to_account_info())
    .update_authority(&ctx.accounts.payer.to_account_info())
    .mint_authority(&ctx.accounts.payer.to_account_info())
    .payer(&ctx.accounts.payer.to_account_info())
    .metadata(&ctx.accounts.metadata_account.to_account_info())
    .token_program(&ctx.accounts.token_program.to_account_info())
    .system_program(&ctx.accounts.system_program.to_account_info())
    .rent(Some(&ctx.accounts.rent.to_account_info()))
    .invoke();
    Ok(())
}

#[derive(Accounts)]
pub struct GHH<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Address validated using constraint
    #[account(
        mut,
        // address=find_metadata_account(&mint_account.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,
    // CHECK: Address validated using constraint
    #[account(
        init,
        payer = payer,
        space = 8,
        // address=create_pda(mint_account.key(),242)
        // address=MasterEdition::
        // seeds  = [
        //     b"metadata",
        //     token_metadata_program.key().as_ref(),
        //     mint_account.key().as_ref(),
        //     b"edition",
        //     ],
        // bump,

        owner = token_metadata_program.key(),
    )]
    pub edition_account: UncheckedAccount<'info>,
    // Create new mint account, NFTs have 0 decimals
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    // Create associated token account, if needed
    // This is the account that will hold the NFT
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>, // actully no need
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    // pub sysvar_instructions: UncheckedAccount<'info>, // Sysvar1nstructions1111111111111111111111111
}

///////////////////// Working mint function ////////////////////////
// pub fn create_next_thing(ctx: Context<GHH>) -> Result<()> {
//     let mint_cpi = MintV1CpiBuilder::new(&ctx.accounts.token_metadata_program.to_account_info())
//         .token(&ctx.accounts.associated_token_account.to_account_info())
//         .token_owner(Some(&ctx.accounts.payer.to_account_info()))
//         .metadata(&ctx.accounts.metadata_account.to_account_info())
//         .master_edition(Some(&ctx.accounts.edition_account.to_account_info()))
//         .mint(&ctx.accounts.mint_account.to_account_info())
//         .payer(&ctx.accounts.payer.to_account_info())
//         .authority(&ctx.accounts.payer.to_account_info())
//         .system_program(&ctx.accounts.system_program.to_account_info())
//         .sysvar_instructions(&ctx.accounts.sysvar_instructions.to_account_info())
//         .spl_token_program(&ctx.accounts.token_program.to_account_info())
//         .spl_ata_program(&ctx.accounts.associated_token_program.to_account_info())
//         .amount(1)
//         .invoke();
//     Ok(())
// }
