// use mpl_token_metadata::accounts::MasterEdition;

use anchor_lang::accounts::system_account::SystemAccount;
use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        metadata::Metadata,
        token::{Mint, Token, TokenAccount},
    },
    mpl_token_metadata::{
        accounts::MasterEdition,
        instructions::{
            CreateMasterEditionV3Cpi, CreateMasterEditionV3CpiAccounts,
            CreateMasterEditionV3InstructionArgs,
        },
    },
};
// programs::MPL_TOKEN_METADATA_ID,
pub fn create_master_edition_account(ctx: Context<Master>) -> Result<()> {
    let bump = ctx.bumps.edition_account;

    msg!("{} ===========", bump);
    let editon2 = MasterEdition::create_pda(ctx.accounts.mint_account.key(), bump);

    msg!("{:#?} ===========", editon2);

    let editon = MasterEdition::find_pda(&ctx.accounts.mint_account.key());
    msg!("{} =========== {}", editon.0, editon.1);

    ///////////////////////////////////////////////////////////////

    msg!("----------------------------------------------");
    let token_meta_prg = ctx.accounts.token_metadata_program.key();
    let mint_acc = ctx.accounts.mint_account.key();
    let seeds = [
        b"metadata",
        token_meta_prg.as_ref(),
        mint_acc.as_ref(),
        b"edition",
        token_meta_prg.as_ref(),
        &[bump],
    ];
    msg!("----------------------------------------------");
    let signer_seeds = &[&seeds[..]];
    msg!("signer seeds are : {:?} =====", signer_seeds); // do not use {:#?}
    msg!("===============================================");
    let master_edition = CreateMasterEditionV3Cpi::new(
        &ctx.accounts.token_metadata_program.to_account_info(),
        CreateMasterEditionV3CpiAccounts {
            edition: &ctx.accounts.edition_account.to_account_info(),
            mint: &ctx.accounts.mint_account.to_account_info(),
            update_authority: &ctx.accounts.payer.to_account_info(),
            mint_authority: &ctx.accounts.payer.to_account_info(),
            payer: &ctx.accounts.payer.to_account_info(),
            metadata: &ctx.accounts.metadata_account.to_account_info(),
            token_program: &ctx.accounts.token_metadata_program.to_account_info(),
            system_program: &ctx.accounts.system_program.to_account_info(),
            rent: Some(&ctx.accounts.rent.to_account_info()),
        },
        CreateMasterEditionV3InstructionArgs {
            max_supply: Some(0),
        },
    ).invoke_signed(signer_seeds);
    // &[&[&[u8]]]

    // master_edition.invoke_signed();
    msg!("NFT minted successfully.");

    Ok(())
}

// use anchor_spl::metadata::create_master_edition_v3;

#[derive(Accounts)]
pub struct Master<'info> {
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
        
        owner = token_metadata_program.key(),
        seeds  = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            mint_account.key().as_ref(),
            b"edition",
            // token_metadata_program.key().as_ref(), // added to verify
            ],

        // seeds::program = token_metadata_program.key(), // new concept
        bump,
    )]
    pub edition_account: UncheckedAccount<'info>,
    // pub edition_account: UncheckedAccount<'info>,
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
    pub associated_token_account: Account<'info, TokenAccount>, // actully needed for whatever reason
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// create_master_edition_v3(
//     CpiContext::new(
//         ctx.accounts.token_metadata_program.to_account_info(),
//         CreateMasterEditionV3 {
//             edition: ctx.accounts.edition_account.to_account_info(),
//             mint: ctx.accounts.mint_account.to_account_info(),
//             update_authority: ctx.accounts.payer.to_account_info(),
//             mint_authority: ctx.accounts.payer.to_account_info(),
//             payer: ctx.accounts.payer.to_account_info(),
//             metadata: ctx.accounts.metadata_account.to_account_info(),
//             token_program: ctx.accounts.token_program.to_account_info(),
//             system_program: ctx.accounts.system_program.to_account_info(),
//             rent: ctx.accounts.rent.to_account_info(),
//         },
//     ),
//     Some(0), // Max Supply
// )?;
